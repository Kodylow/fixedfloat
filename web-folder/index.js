let isLoading = false;

// Get DOM elements
const toggle = document.getElementById("toggle");
const select = document.getElementById("currency-codes");
const address = document.getElementById("address");
const form = document.getElementById("currency-form");
const responseDiv = document.querySelector(".response");
responseDiv.classList.add("hidden");

const updateLoadingState = () => {
  if (isLoading) {
    form.classList.add("hidden");
    responseDiv.classList.remove("hidden");
    responseDiv.textContent = "Loading...";
  } else {
    form.classList.remove("hidden");
    responseDiv.classList.add("hidden");
    responseDiv.textContent = "";
  }
};

// Constants
const API_URL = "http://localhost:8080/api";
const SEND = "send";
const RECEIVE = "receive";
const COMPLETED = "completed";

let currenciesData = null;

// Functions
const justUSDC = () => {
  select.innerHTML = `<option value="USDCETH">USDCETH</option>`;
};

const populateCurrencies = (data, type) => {
  select.innerHTML = "";
  const filteredData = data.data.filter((item) => {
    const isAvailable = type === "send" ? item.send : item.recv;
    return Number(isAvailable) === 1;
  });

  filteredData.forEach((item) => {
    const option = document.createElement("option");
    option.value = item.code;
    option.text = item.code;
    select.appendChild(option);
  });
};

const fetchData = async (url, options) => {
  try {
    const response = await fetch(url, options);
    return await response.json();
  } catch (error) {
    console.error("Error:", error);
  }
};

const fetchCurrencies = async () => {
  const data = await fetchData(`${API_URL}/currencies`);
  justUSDC();
};

const handleToggleChange = () => {
  address.style.display = toggle.value === "send" ? "block" : "none";
  address[toggle.value === "send" ? "setAttribute" : "removeAttribute"](
    "required",
    ""
  );

  responseDiv.textContent = "";
  responseDiv.style.display = "none";

  if (currenciesData) {
    populateCurrencies(currenciesData, toggle.value);
  }
};

const pollOrderDetails = async (id, token, statusMessage) => {
  const pollInterval = setInterval(async () => {
    try {
      const data = await fetchData(`${API_URL}/order-details`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          id,
          token,
        }),
      });
      console.log("Order details:", data);

      // Update status message
      statusMessage.textContent = `Status: ${data["data"]["status"]}`;

      // Stop polling if order is completed
      if (data["data"]["status"] === "completed") {
        clearInterval(pollInterval);
      }
    } catch (error) {
      console.error("Error:", error);
    }
  }, 15000);
};

const handleSend = async (toggleValue, currencyCode, amount, addressValue) => {
  // Create request body for "send"
  const requestBody = {
    direction: "to",
    ccy: currencyCode,
    amount: amount,
    toAddress: addressValue,
  };

  console.log("Request body:", requestBody); // logging request body

  const data = await fetchData(`${API_URL}/create-order`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  // If webln is available, enable it and send payment
  await handleSendPayment(data);
};

const handleReceive = async (toggleValue, currencyCode, amount) => {
  // Create request body for "receive"
  const requestBody = {
    direction: "from",
    ccy: currencyCode,
    amount: amount,
  };

  console.log("Request body:", requestBody); // logging request body

  const data = await fetchData(`${API_URL}/exchange-rate`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  let btcAmount = parseFloat(data["data"]["to"]["amount"]);
  let amountInSatoshis = btcAmount * 100000000;

  await handleReceivePayment(amountInSatoshis, currencyCode, btcAmount, data);
};

const handleFormSubmit = async (event) => {
  event.preventDefault();
  isLoading = true;
  updateLoadingState();

  // Get form values
  const toggleValue = toggle.value;
  const currencyCode = select.value;
  const amount = document.getElementById("amount").value;
  const addressValue = address.value;

  if (toggleValue === "send") {
    await handleSend(toggleValue, currencyCode, amount, addressValue);
  } else if (toggleValue === "receive") {
    await handleReceive(toggleValue, currencyCode, amount);
  }

  isLoading = false;
  updateLoadingState();
};

// Event listeners
toggle.addEventListener("change", handleToggleChange);
form.addEventListener("submit", handleFormSubmit);

// Initial setup
window.onload = async function () {
  address.style.display = toggle.value === "send" ? "block" : "none";
  await fetchCurrencies();
  updateLoadingState();
};

async function handleReceivePayment(
  amountInSatoshis,
  currencyCode,
  btcAmount,
  data
) {
  if (typeof window.webln !== "undefined") {
    await window.webln.enable();
    const { paymentRequest } = await window.webln.makeInvoice(amountInSatoshis);

    if (!!paymentRequest) {
      await createReceiveOrder(currencyCode, btcAmount, paymentRequest);
    } else {
      console.log("Failed to create invoice!");
    }
  } else {
    // alert if webln is not available
    alert("WebLN is not available!");
  }
}

async function createReceiveOrder(currencyCode, btcAmount, paymentRequest) {
  const requestBody = {
    direction: "from",
    ccy: currencyCode,
    amount: String(btcAmount),
    toAddress: paymentRequest,
  };

  const data = await fetchData(`${API_URL}/create-order`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });
  console.log("Data Order:", data);

  const statusMessage = showReceiveInfo(data);

  await pollOrderDetails(
    (id = data["data"]["id"]),
    (token = data["data"]["token"]),
    statusMessage
  );
}

async function generateQRCode(data) {
  const response = await fetch(`${API_URL}/qrcode`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ data }),
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return await response.text();
}

async function showReceiveInfo(data) {
  const ethURI = `ethereum:${data["data"]["from"]["address"]}?value=${data["data"]["from"]["amount"]}&token=${data["data"]["from"]["coin"]}`;

  // Generate QR code
  const qrSvg = await generateQRCode(ethURI);

  // Create a container for the QR code
  const qrCodeContainer = document.createElement("div");
  qrCodeContainer.innerHTML = qrSvg;

  // Create a container for the card box
  const cardBox = document.createElement("div");
  cardBox.classList.add("card-box");

  // Create copy button
  const copyButton = document.createElement("button");
  copyButton.textContent = `Copy ${data["data"]["from"]["coin"]} URI`;
  copyButton.addEventListener("click", async () => {
    await navigator.clipboard.writeText(ethURI);
  });

  // Show payment message
  const paymentMessage = document.createElement("p");
  paymentMessage.textContent = `Pay \n${data["data"]["from"]["amount"]} ${data["data"]["from"]["coin"]} \nto \n${data["data"]["from"]["address"]}`;

  // Show warning message
  const warningMessage = document.createElement("p");
  warningMessage.textContent = `Pay EXACTLY ${data["data"]["from"]["amount"]} ${data["data"]["from"]["coin"]} or your stupid shitcoin will be lost forever.`;
  warningMessage.classList.add("warning");

  // Show status
  const statusMessage = document.createElement("p");

  // Append the QR code container, copy button, payment message, and warning message to the card box
  cardBox.appendChild(qrCodeContainer);
  cardBox.appendChild(copyButton);
  cardBox.appendChild(paymentMessage);
  cardBox.appendChild(warningMessage);
  cardBox.appendChild(statusMessage);

  // Append the card box to the body
  document.body.appendChild(cardBox);

  return statusMessage;
}

async function handleSendPayment(data) {
  if (typeof window.webln !== "undefined") {
    await window.webln.enable();
    const invoice = data["data"]["from"]["address"];
    console.log("Invoice:", invoice);
    let { preimage } = await window.webln.sendPayment(invoice);

    if (!!preimage) {
      console.log("Payment successful!");
    } else {
      console.log("Payment failed!");
    }

    await pollOrderDetails(
      (id = data["data"]["id"]),
      (token = data["data"]["token"])
    );
  } else {
    // Append response to the DOM
    const responseDiv = document.createElement("div");
    responseDiv.textContent = JSON.stringify(data);
    document.body.appendChild(responseDiv);
  }
}
