<template>
  <div>
    <form @submit.prevent="submitForm">
      <label for="cardholder">Cardholder Name:</label>
      <input
        type="text"
        id="cardholder"
        v-model="cardholderName"
        :class="{ 'error': errors.cardholder }"
        placeholder="Enter cardholder name"
      />
      <label for="cardnumber">Card Number:</label>
      <input
        type="text"
        id="cardnumber"
        v-model="cardNumber"
        :class="{ 'error': errors.cardNumber }"
        :maxlength="19"
        @input="formatCardNumber"
        placeholder="Enter card number (e.g., 1234-5678-9012-3456)"
      />
      <label for="expiry">Expiry Date:</label>
      <input
        type="text"
        id="expiry"
        v-model="expiryDate"
        :class="{ 'error': errors.expiryDate }"
        :maxlength="5"
        @input="formatExpiryDate"
        placeholder="Enter expiry date (e.g., 12/23)"
      />
      <label for="cvv">CVV:</label>
      <input
        type="text"
        id="cvv"
        v-model="cvv"
        :class="{ 'error': errors.cvv }"
        :maxlength="3"
        @input="formatCVV"
        placeholder="Enter CVV (e.g., 123)"
      />
      <button type="submit">Pay Now</button>
    </form>

    <div class="payment-result" v-if="paymentResult">
      <h2>Payment Result:</h2>
      <p>{{ paymentResult }}</p>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue';

import store from '../components/extra_javascript/store.js';

export default {
  setup() {
    const cardholderName = ref("");
    const cardNumber = ref("");
    const expiryDate = ref("");
    const cvv = ref("");
    const errors = ref({
      cardholder: false,
      cardNumber: false,
      expiryDate: false,
      cvv: false,
    });
    const paymentResult = ref("");
    const paymentAmount = ref(0);

    const submitForm = () => {
      console.log(paymentAmount.value);
      console.log(selectedMembership.value);
      // Check for empty fields
      errors.value.cardholder = cardholderName.value.trim() === "";
      errors.value.cardNumber = cardNumber.value.trim() === "";
      errors.value.expiryDate = expiryDate.value.trim() === "";
      errors.value.cvv = cvv.value.trim() === "";

      if (!Object.values(errors.value).some((error) => error)) {
        // All fields are filled, proceed with payment
        // Perform payment processing logic here
        paymentResult.value = "Payment successful!";
      }
    };

    const formatCardNumber = () => {
      cardNumber.value = cardNumber.value
        .replace(/\D/g, "")
        .replace(/(.{4})/g, "$1-")
        .trim()
        .slice(0, 19);
    };

    const formatExpiryDate = () => {
      expiryDate.value = expiryDate.value
        .replace(/\D/g, "")
        .replace(/(.{2})/, "$1/")
        .trim()
        .slice(0, 5);
    };

    const formatCVV = () => {
      cvv.value = cvv.value.replace(/\D/g, "").trim().slice(0, 3);
    };

    paymentAmount.value = store.getPaymentAmount();
    const selectedMembership = ref(store.getSelectedMembership());

    return {
      cardholderName,
      cardNumber,
      expiryDate,
      cvv,
      errors,
      paymentResult,
      paymentAmount,
      submitForm,
      formatCardNumber,
      formatExpiryDate,
      formatCVV,
      selectedMembership,
    };
  },
};
</script>

<style scoped>
form {
  max-width: 400px;
  margin: 0 auto;
  padding: 20px;
  border: none;
  border-radius: 10px;
  background-color: #f9f9f9;
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
}

form label {
  display: block;
  margin-bottom: 10px;
  font-weight: bold;
  color: #333;
}

form input[type="text"] {
  width: 100%;
  padding: 10px;
  margin-bottom: 15px;
  border: none;
  border-radius: 5px;
  background-color: #f5f5f5;
  color: #333;
  font-size: 16px;
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.1);
}

form input[type="text"]:focus {
  outline: none;
  background-color: #ebebeb;
}

form button[type="submit"] {
  display: inline-block;
  padding: 10px 20px;
  background-color: #007bff;
  color: #fff;
  border: none;
  border-radius: 5px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.3s;
}

form button[type="submit"]:hover {
  background-color: #0056b3;
}

div.payment-result {
  margin-top: 20px;
}

div.payment-result h2 {
  font-size: 18px;
  color: #007bff;
}

div.payment-result p {
  margin-top: 10px;
  color: #333;
}

form input.error {
  border-color: red;
  background-color: #ffd8d8;
}

form label.error {
  color: red;
  font-size: 12px;
}

/* Custom Styling */
form {
  background-color: #fff;
  border: 1px solid #ebebeb;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

form input[type="text"] {
  border: 1px solid #ebebeb;
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.1);
}

form button[type="submit"] {
  background-color: #007bff;
  color: #fff;
  font-weight: bold;
}

form button[type="submit"]:hover {
  background-color: #0056b3;
}

div.payment-result {
  background-color: #f9f9f9;
  border: 1px solid #ebebeb;
  border-radius: 5px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}
</style>
