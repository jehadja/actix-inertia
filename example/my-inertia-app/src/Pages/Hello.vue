<template>
  <div>
    <h1>{{ message }}</h1>
    <Link as="button" href="/world" title="another page"> Go To World </Link>
  </div>
</template>

<script setup>
import { Link } from "@inertiajs/inertia-vue3";
import { ref } from "vue";

defineProps({
  message: String,
});

const count = ref(0);
</script>
