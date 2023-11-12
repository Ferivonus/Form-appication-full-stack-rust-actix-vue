<script>
export default {
  props: {
    hizmet: {
      type: Object,
      required: true
    }
  },
  methods: {
    toggleRecommendedUsers() {
      this.hizmet.showRecommendedUsers = !this.hizmet.showRecommendedUsers;
    }
  }
}
</script>

<template>
    <div class="ozelders">
      <h2>{{ hizmet.baslik }}</h2>
      <p>{{ hizmet.aciklama }}</p>
      <p>Most Enrolled User: {{ hizmet.mostEnrolledUser }}</p>
      <p>Highest Rated User: {{ hizmet.highestRatedUser }}</p>
  
      <!-- Özel ders seçimi için form -->
      <div>
        <label for="dersSecimi">Ders Seçimi:</label>
        <select v-model="dersSecimi" id="dersSecimi">
          <option value="tm">TM</option>
          <option value="mf">MF</option>
        </select>
      </div>
  
      <!-- Seçime göre derslerin görüntülenmesi -->
      <div v-if="dersSecimi === 'tm'">
        <h3>TM Dersleri:</h3>
        <ul>
          <li>Türkçe</li>
          <li>Sosyal Bilimler</li>
          <!-- Diğer TM dersleri -->
        </ul>
      </div>
  
      <div v-if="dersSecimi === 'mf'">
        <h3>MF Dersleri:</h3>
        <ul>
          <li>Matematik</li>
          <li>Fizik</li>
          <!-- Diğer MF dersleri -->
        </ul>
      </div>
    </div>
  </template>
  
  <style scoped>
  </style>
  