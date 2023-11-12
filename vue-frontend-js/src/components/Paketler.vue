<script setup>
import { reactive } from 'vue'
import store from '../components/extra_javascript/store.js';

const uyelikPaketleri = reactive([
  {
    baslik: 'Bronz Üyelik',
    aciklama: 'Bronz üyelik paketi ile aşağıdaki avantajlardan yararlanabilirsiniz:',
    Detailes: [
      'Temel derslere erişim',
      'Eğitim materyallerine sınırlı erişim',
      'İndirimli psikolog danışmanlık',
    ],
    ucret: {
      aylık: '9.99 TL',
      yillik: '99.99 TL'
    },
    imagePath: "/src/assets/bronze-membership.svg",
  },
  {
    baslik: 'Silver Üyelik',
    aciklama: 'Silver üyelik paketi ile aşağıdaki avantajlardan yararlanabilirsiniz:',
    Detailes: [
      'Belirli derslere sınırlı erişim',
      'Eğitim materyallerine sınırlı erişim',
      'İndirimli psikolog danışmanlık',
      'Haftada 1 ücretsiz özel ders',
    ],
    ucret: {
      aylık: '19.99 TL',
      yillik: '199.99 TL'
    },
    imagePath: "/src/assets/silver-membership.svg",
  },
  {
    baslik: 'Gold Üyelik',
    aciklama: 'Gold üyelik paketi ile aşağıdaki avantajlardan yararlanabilirsiniz:',
    Detailes: [
      'Sınırsız özel derslere erişim',
      'Eğitim materyallerine erişim',
      'İndirimli psikolog danışmanlık',
      'Uzman mentorlük hizmeti',
      'Haftada 2 ücretsiz özel ders',
    ],
    ucret: {
      aylık: '29.99 TL',
      yillik: '299.99 TL'
    },
    imagePath: "/src/assets/gold-membership.svg",
  },
  {
    baslik: 'Platin Üyelik',
    aciklama: 'Platin üyelik paketi ile aşağıdaki avantajlardan yararlanabilirsiniz:',
    Detailes: [
      'Tüm derslere sınırsız erişim',
      'Eğitim materyallerine erişim',
      'İndirimli psikolog danışmanlık',
      'Kişisel uzman mentorlük hizmeti',
      'Haftada 3 ücretsiz özel ders',
      'Özel etkinliklere öncelikli katılım',
    ],
    ucret: {
      aylık: '49.99 TL',
      yillik: '499.99 TL'
    },
    imagePath: "/src/assets/platinum-membership.svg",
  },
  {
    baslik: 'Elmas Üyelik',
    aciklama: 'Elmas üyelik paketi ile aşağıdaki avantajlardan yararlanabilirsiniz:',
    Detailes: [
      'Tüm derslere sınırsız erişim',
      'Eğitim materyallerine erişim',
      'İndirimli özel dersler',
      'Mentörlük hizmetinden faydalanma',
      'Psikolog danışmanlık indirimi',
      'Öncelikli etkinliklere katılım',
    ],
    ucret: {
      aylık: '49.99 TL',
      yillik: '499.99 TL'
    },
    imagePath: "/src/assets/diamond-membership.svg",
  },
  {
    baslik: 'Platin Plus Üyelik',
    aciklama: 'Platin Plus üyelik paketi ile aşağıdaki avantajlardan yararlanabilirsiniz:',
    Detailes: [
      'Tüm derslere sınırsız erişim',
      'Eğitim materyallerine erişim',
      'İndirimli özel dersler',
      'Mentörlük hizmetinden faydalanma',
      'Psikolog danışmanlık indirimi',
      'Öncelikli etkinliklere katılım',
      'Haftada 4 ücretsiz özel ders',
    ],
    ucret: {
      aylık: '49.99 TL',
      yillik: '499.99 TL'
    },
    imagePath: "/src/assets/platinum-plus-membership.svg",
  }
  
]);

const setSecim = (uyelik, secim) => {
  uyelik.secim = secim;
};

const proceedToPayment = () => {
  const selectedMembership = uyelikPaketleri.find(uyelik => uyelik.secim);
  if (selectedMembership) {
    const paymentAmount = selectedMembership.secim === 'aylik' ? selectedMembership.ucret.aylık : selectedMembership.ucret.yillik;
    store.setPaymentAmount(paymentAmount);
    store.setSelectedMembership(selectedMembership); // Seçilen üyelik paketini store üzerine kaydet
    console.log(store.getPaymentAmount());
  }
};


</script>

<template>

<div class="Neden-üyelikler-önemli">
    <div class="indirim-yazisi">
      <h3>Neden Üyelikler Önemli?</h3>
      <p>Üyeliklerimiz size birçok avantaj sağlar:</p>
      <ul>
        <li>Geniş ders seçeneklerine sınırsız erişim</li>
        <li>Özel derslerde indirimli fiyatlar</li>
        <li>Mentörlük hizmetinden yararlanma imkanı</li>
        <li>Psikolog danışmanlık indirimi</li>
        <li>Öncelikli etkinliklere katılım fırsatı</li>
      </ul>
      <p>Üstelik, aylık veya yıllık üyelik seçenekleriyle ihtiyaçlarınıza uygun şekilde seçim yapabilirsiniz.</p>
      <p>Hemen siz de avantajlı üyeliklerimizden faydalanmaya başlayın!</p>
    </div>
  </div>

  <div class="uyeliklerimiz">
    <div v-for="uyelik in uyelikPaketleri" :key="uyelik.baslik" class="uyelik-item">
      <div class="uyelik-content">
        <div class="uyelik-picture">
          <img :src="uyelik.imagePath" alt="Resim" />
        </div>
        <div class="uyelik-details">
          <h3>{{ uyelik.baslik }}</h3>
          <ul class="avantajlar">
            <li v-for="detail in uyelik.Detailes" :key="detail"><span>{{ detail }}</span></li>
          </ul>
          <div class="fiyatlar">
            <button class="secim-button" :class="{ active: uyelik.secim === 'aylik' }" @click="setSecim(uyelik, 'aylik')">Aylık: {{ uyelik.ucret.aylık }}</button>
            <button class="secim-button" :class="{ active: uyelik.secim === 'yillik' }" @click="setSecim(uyelik, 'yillik')">Yıllık: {{ uyelik.ucret.yillik }}</button>
          </div>
          <router-link class="satinal-button" @click="proceedToPayment" to="/satinalma" rel="noopener" >Satın Al</router-link>
        </div>
      </div>
    </div>
  </div>
  </template>
  
  
  
  
  
  <style scoped>

  .Neden-üyelikler-önemli {
  margin-bottom: 20px;
}

.indirim-yazisi {
  background-color: #f5f5f5;
  padding: 20px;
  border-radius: 10px;
  max-width: 600px;
  margin: 0 auto;
}

.indirim-yazisi h3 {
  font-size: 24px;
  margin-bottom: 10px;
  color: indianred;
  text-align: center;

}

.indirim-yazisi p {
  margin-bottom: 10px;
}

.indirim-yazisi ul {
  margin: 0;
  padding: 0;
  list-style-type: disc;
  margin-left: 0px;
  margin: 10px;
}

.indirim-yazisi ul li {
  margin-bottom: 5px;
  color: green;
}

.indirim-yazisi p:last-child {
  margin-top: 20px;
}
  .uyeliklerimiz {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
  }
  
  .avantajlar {
    list-style-type: none;
    padding-left: 0;
    margin-top: 10px;
    color:rgb(197, 0, 0);
  }
  
  .avantajlar li {
    display: flex;
    align-items: center;
    margin-bottom: 8px;
  }
  
  .avantajlar li:before {
    content: "•";
    color: #333;
    margin-right: 8px;
  }
  
  .uyelik-item {
    flex-basis: calc(33.33% - 20px);
    margin: 10px;
    background-color: #f5f5f5;
    border-radius: 10px;
    padding: 20px;
    box-shadow: 4px 2px 4px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s;
  }
  
  .uyelik-item:hover {
    transform: translateY(-5px);
  }
  
  .uyelik-content {
    display: flex;
    align-items: center;
  }
  
  .uyelik-picture {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    background-color: indianred;
    margin-right: 20px;
  }
  
  .uyelik-picture img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 50%;
  }
  
  .uyelik-details {
    flex: 1;
  }
  
  .uyelik-details h3 {
    font-size: 20px;
    margin: 0;
    color: #333;
  }

  .uyelik-details .fiyatlar {
    margin-top: 20px;
  }

  .uyelik-details .secim-button {
    display: inline-block;
    margin-right: 10px;
    padding: 5px 10px;
    background-color: transparent;
    border: none;
    color: #333;
    font-weight: bold;
    cursor: pointer;
    transition: color 0.3s;
    border-radius: 5px;
    border: 1px solid #333;
    margin: 5px;
  }

  .uyelik-details .secim-button.active {
    color: green;
    background-color: green;
    color: white;
  }

  .uyelik-details .satinal-button {
    display: inline-block;
    margin-top: 20px;
    padding: 10px 20px;
    background-color: #333;
    color: #fff;
    text-decoration: none;
    border-radius: 5px;
    transition: background-color 0.3s;
  }

  .uyelik-details .satinal-button:hover {
    background-color: #555;
  }
</style>