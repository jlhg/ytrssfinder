<script setup lang="ts">
  // console.log(import.meta.env.VITE_SERVER_API_URL)
  import Search from './components/Search.vue'
</script>

<template>
  <q-layout view="hHh lpR fFf">
    <q-page-container>
      <Search />
    </q-page-container>
    <q-footer class="bg-white text-blue-grey-4 text-center">
      <div class="q-pa-md">
        <div class="q-gutter-md">
          <div class="row justify-center">
            <div class="col-md-6 col-xs-12">
              <p>
                <a href="https://github.com/jlhg/ytrssfinder" target="_blank"><q-icon name="fa-brands fa-github" size="sm" /></a>
              </p>
              <p>
                Released under the <a href="https://opensource.org/license/mit/" target="_blank">MIT License</a>.<br>
                Copyright © 2023 Jian-Long Huang<br>
              </p>
            </div>
          </div>
        </div>
      </div>
    </q-footer>
  </q-layout>
</template>

<style scoped>
</style>
