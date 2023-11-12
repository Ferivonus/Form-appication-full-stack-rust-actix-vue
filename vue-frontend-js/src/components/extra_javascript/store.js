const store = (() => {
  const state = {
    paymentAmount: 0,
    selectedMembership: null,
  };

  const setPaymentAmount = (amount) => {
    state.paymentAmount = amount;
  };

  const getPaymentAmount = () => {
    return state.paymentAmount;
  };

  const setSelectedMembership = (membership) => {
    state.selectedMembership = membership;
  };

  const getSelectedMembership = () => {
    return state.selectedMembership;
  };

  return Object.freeze({
    setPaymentAmount,
    getPaymentAmount,
    setSelectedMembership,
    getSelectedMembership,
  });
})();

export default store;
