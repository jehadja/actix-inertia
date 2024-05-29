<template>
  <div>
    <h1>{{ message }}</h1>
    <div style="color: blue" @click="gotopage" title="another page"></div>
    <Link href="/version" :headers="{ 'X-Inertia-Version': '1' }" as="button">
      Go To Middleware Version Page
    </Link>
  </div>
</template>

<script setup>
import { Link } from "@inertiajs/inertia-vue3";

const props = defineProps({
  message: String,
});
</script>
