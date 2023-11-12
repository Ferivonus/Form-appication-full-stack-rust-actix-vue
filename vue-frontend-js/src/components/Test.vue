<template>
  <div>
    <h1>{{ count }}</h1>
    <h1>hi 1 </h1>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';

const count = ref(0);
let timer;

if (typeof window !== 'undefined') {
  onMounted(() => {
    timer = setInterval(() => {
      count.value++;
    }, 1000);
  });

  onUnmounted(() => {
    clearInterval(timer);
  });
}
</script>
