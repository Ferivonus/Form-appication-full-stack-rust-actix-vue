<script setup>
import { reactive } from 'vue'


const hizmetler = reactive([
  {
    baslik: 'Mentörlük',
    aciklama: 'Mentörlük hizmetimizle öğrencilere akademik başarılarında rehberlik ediyoruz. Uzman mentorlarımız, öğrencilerin hedeflerini belirlemelerine yardımcı olur, çalışma stratejileri geliştirmelerine destek sağlar ve motivasyonlarını artırmaya yardımcı olur.',
    Detailes: 'Mentorlüğün detayları burada yazıyor olacak.',
    link: '/hizmetler/mentorlar',
    imagePath: "/src/assets/logo.svg",
    mostEnrolledUser: 'Fahretin',
    highestRatedUser: 'Fahretin',
    showRecommendedUsers: false,
    showDetails: false
  },
  {
    baslik: 'Özel Ders',
    aciklama: 'Özel ders hizmetimizle fizik, kimya, biyoloji ve matematik gibi temel derslerde öğrencilere birebir ders imkanı sunuyoruz. Uzman öğretmenlerimiz öğrencilerin ihtiyaçlarına uygun ders programları oluşturur, eksikliklerini gidermelerine yardımcı olur ve konuları daha iyi anlamalarını sağlar.',
    Detailes: 'Özel Derslerin detayları burada yazıyor olacak.',
    link: '/hizmetler/ozeldersSSS',
    imagePath: "/src/assets/logo.svg",
    mostEnrolledUser: 'Fahretin',
    highestRatedUser: 'Fahretin',
    showRecommendedUsers: false,
    showDetails: false
  },
  {
    baslik: 'Koçluk',
    aciklama: 'Psikolog hizmetimizle öğrencilerin psikolojik sağlıklarını destekliyoruz. Deneyimli psikologlarımız, öğrencilerle birebir görüşmeler yaparak stres, kaygı, motivasyon gibi konularda yardımcı olur ve duygusal refahlarını artırmaya yönelik stratejiler sunar.',
    Detailes: 'Psikologun detayları burada yazıyor olacak.',
    link: '/hizmetler/psikologlar',
    imagePath: "/src/assets/logo.svg",
    mostEnrolledUser: 'Fahretin',
    highestRatedUser: 'Fahretin',
    showRecommendedUsers: false,
    showDetails: false
  },
  {
    baslik: 'Psikolog',
    aciklama: 'Psikolog hizmetimizle öğrencilerin psikolojik sağlıklarını destekliyoruz. Deneyimli psikologlarımız, öğrencilerle birebir görüşmeler yaparak stres, kaygı, motivasyon gibi konularda yardımcı olur ve duygusal refahlarını artırmaya yönelik stratejiler sunar.',
    Detailes: 'Psikologun detayları burada yazıyor olacak.',
    link: '/hizmetler/psikologlar',
    imagePath: "/src/assets/logo.svg",
    mostEnrolledUser: 'Fahretin',
    highestRatedUser: 'Fahretin',
    showRecommendedUsers: false,
    showDetails: false
  }
  
]);

const toggleRecommendedUsers = (hizmet) => {
  hizmet.showRecommendedUsers = !hizmet.showRecommendedUsers;
};

const toggleDetails = (hizmet) => {
  hizmet.showDetails = !hizmet.showDetails;
};

</script>

<template>

<div class="background-image-hizmetler-part">
    <div class="background-image-overlay"></div>
    <div class="hizmetlerin-onemi">
      <h3>Hizmetlerin Önemi</h3>
      <p>Bu hizmetlerin alınması birçok avantaj sağlar. Hizmetlerimiz, sizlere özel olarak tasarlanmış çözümler sunar ve ihtiyaçlarınıza uygun şekilde hizmet verir. Ayrıca, profesyonel ekibimiz size rehberlik eder ve sorularınızı yanıtlar. Hizmetlerimizden yararlanarak verimliliği artırabilir, daha hızlı sonuçlar elde edebilir ve başarıya ulaşabilirsiniz.</p>
      <p>Hizmetlerimizin sunduğu avantajlardan bazıları şunlardır:</p>
      <ul>
        <li>Yüksek kaliteli hizmetler</li>
        <li>Uzman danışmanlık</li>
        <li>Özelleştirilmiş çözümler</li>
        <li>Teknolojik yeniliklerden faydalanma</li>
        <li>Zaman ve maliyet tasarrufu</li>
      </ul>
      <p>Unutmayın, hizmetlerimiz sizin başarınıza katkı sağlamak için burada!</p>
    </div>
  </div>
  <div class="hizmetlerimiz">
    <div v-for="hizmet in hizmetler" :key="hizmet.baslik" class="hizmet-item">
      <div class="hizmet-content">
        <div class="hizmet-title">
          <a :href="hizmet.link">
            <h3>{{ hizmet.baslik }}</h3>
          </a>
        </div>
        <div class="hizmet-bottom">
          <a :href="hizmet.link">
            <div class="hizmet-picture red">
              <img :src="hizmet.imagePath" alt="Resim" />
            </div>
          </a>
          <div class="hizmet-description">
            <a :href="hizmet.link">
              <p>{{ hizmet.aciklama }}</p>
            </a>
            <transition name="fade">
              <div v-if="hizmet.showRecommendedUsers" class="recommended-users">
                <p>Most Enrolled User: {{ hizmet.mostEnrolledUser }}</p>
                <p>Highest Rated User: {{ hizmet.highestRatedUser }}</p>
              </div>
            </transition>
          </div>
        </div>
        <div class="hizmet-buttons">
          <button @click="toggleRecommendedUsers(hizmet)">Toggle Recommended Users</button>
          <button @click="toggleDetails(hizmet)">Detayları Göster</button>
        </div>
        <div v-if="hizmet.showDetails" class="hizmet-details">
          <h4>{{ hizmet.baslik }} Detayları:</h4>
          <p>{{ hizmet.Detailes }}</p>
        </div>
      </div>
    </div>
  </div>

  


</template>


<style scoped>

.hizmet-details, .hizmet-details p {
  color: #333;
  text-align: center;
}
.hizmetlerimiz {
  margin: 0px auto;
}
.hizmetlerimiz a {
    text-decoration: none; 
}
.hizmet-item {
  margin-bottom: 20px;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 10px;
  display: flex;
  box-shadow: 4px 2px 4px rgba(0, 0, 0, 0.1);
}

.hizmet-content {
  display: flex;
  flex-direction: column;
}

.hizmet-title {
  text-align: center;
}

.hizmet-title h3 {
  font-size: 24px;
  font-weight: bold;
  text-transform: uppercase;
  margin: 0;
  color: #333;

}

.hizmet-bottom {
  display: flex;
  align-items: flex-end;
  margin-top: 10px;
}

.hizmet-picture {
  width: 120px;
  height: 120px;
  margin: auto;
  border-radius: 50%;
  background-color: indianred;
}

.hizmet-description {
  flex: 1;
  margin: auto;
  margin-left: 20px;
  
}

.hizmet-description p {
  text-align: center;
}

.hizmet-buttons {
  margin-top: 10px;
  text-align: center;
}

.hizmet-buttons button {
  margin-right: 10px;
}
.recommended-users p{
color: goldenrod;
margin-top: 10px;
border: 1px solid aquamarine;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s;
  margin-top: 20px;
}

.fade-enter,
.fade-leave-to {
  opacity: 0;
}


/* Arka plan resmi stilleri */
.background-image-hizmetler-part {
  position: relative;
}

.background-image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: url('https://1.bp.blogspot.com/-HkCsPhMqCNQ/VMDK9dzBj8I/AAAAAAAAYyc/1kLYHyQm20w/s1600/dag-hd-resim-rooteto6.jpg');
  background-size: cover;
  opacity: 0.3;
}

.hizmetlerin-onemi {
  margin: 0 auto;
  padding: 40px;
  max-width: 800px;
  background-color: rgba(255, 255, 255, 0.8);
  border-radius: 20px;
  color: #333;
}

.hizmetlerin-onemi h3 {
  font-size: 28px;
  font-weight: bold;
}

.hizmetlerin-onemi p,
.hizmetlerin-onemi ul {
  font-size: 16px;
  line-height: 1.6;
  margin-bottom: 15px;
}

.hizmetlerin-onemi ul {
  list-style-type: disc;
  margin-left: 20px;
  padding-left: 20px;
}

.hizmetlerin-onemi li {
  margin-bottom: 5px;
}

.hizmetlerin-onemi p:last-child {
  margin-bottom: 0;
}
</style>






