<script setup>

const user = {
  name: 'Fahrettin Baştürk',
  avatar: '/src/assets/avatarPhotos/AikoTanaka.jpg'
};
</script>

<template>
  <div class="navigator">
    <nav>
      <ul>
        <li >
          <router-link to="/" rel="noopener">Navigasyon resmi</router-link>
        </li>
        <li >
          <router-link to="/hizmetler" rel="noopener">Hizmetlerimiz</router-link>
        </li>
        <li >
          <router-link to="/paketler" rel="noopener">Paketler</router-link>
        </li>
        <li >
          <router-link to="/vizyon" rel="noopener">Kullanıcı görüşleri</router-link>
        </li>
        <li class="user-profile">
          <router-link to="/h" rel="noopener">
            <img :src="user.avatar" :alt="user.name" />
            <span>{{ user.name }}</span>
          </router-link>
        </li>
      </ul>
    </nav>
  </div>
  <router-view />
</template>



<style scoped>
.navigator {
  padding: 10px;
}

.navigator ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.navigator li {
  display: inline-block;
  margin: 0 10px;
  font-size: 18px;
  position: relative;
}

.navigator a {
  display: block;
  color: #fff;
  text-decoration: none;
  padding: 15px;
  transition: all 0.3s ease;
  border-radius: 5px;
}

.navigator a:hover {
  background-color: #1a4411;
}

.navigator .user-profile {
  display: flex;
  align-items: center;
}

.navigator .user-profile img {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  margin-right: 5px;
}

.navigator .user-profile span {
  font-size: 15px;
  color: #fff;
  align-items: center;
  margin-left: 5px; /* Add margin to create spacing between the image and text */
}

.navigator li.router-link-exact-active a {
  background-color: green;
}

@media screen and (max-width: 800px) {
  .navigator ul {
    flex-wrap: wrap;
    justify-content: center;
  }

  .navigator li {
    display: block;
    margin: 10px 0;
  }
}
</style>
