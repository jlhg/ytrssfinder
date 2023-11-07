<script setup lang="ts">
  import { ref } from 'vue'
  let channelUrl = ref('')
  let pChannelUrl = ref('')
  let feedUrl = ref('')
  let taskStarted = ref(false)
  let searchErrorMsg = ref('')

  function getFeedUrl() {
    if (channelUrl.value == null ||
      channelUrl.value === pChannelUrl.value) {
      return
    }

    searchErrorMsg.value = ''
    taskStarted.value = false
    pChannelUrl.value = channelUrl.value

    let reqUrl = import.meta.env.VITE_SERVER_API_URL + '/' + '?channel_url=' + channelUrl.value
    fetch(reqUrl)
      .then(function(response) {

        switch (response.status) {
        case 200:
          taskStarted.value = true

          response.json().then(data => {
            feedUrl.value = data.data.feed_url
          })

          break
        case 400:
          response.json().then(data => {
            searchErrorMsg.value = data.data.result.message
          })
          break
        }
      })
      .catch(function(e) {
        searchErrorMsg.value = e.toString()
      })
  }
</script>

<template>
  <div class="q-pa-md">
    <div class="q-gutter-md">
      <div class="row justify-center text-center">
        <div class="col-md-6 col-xs-12">
          <h4>YouTube RSS Finder</h4>
        </div>
      </div>

      <div class="row justify-center">
        <div class="col-md-6 col-xs-12">
          <q-input
            v-model.trim="channelUrl"
            outlined
            rounded
            clearable
            clear-icon="fa-solid fa-xmark"
            no-error-icon
            placeholder="Paste the YouTube channel URL. e.g. https://www.youtube.com/user/YouTube/channels"
            v-on:blur="getFeedUrl"
            @keyup.enter="getFeedUrl"
            :error-message="searchErrorMsg"
            :error="searchErrorMsg.length > 0"
          >
            <template v-slot:prepend>
              <q-icon name="fa-solid fa-magnifying-glass" />
            </template>
          </q-input>
        </div>
      </div>

      <div class="row justify-center">
        <div class="col-md-6 col-xs-12">
          <div v-if="taskStarted">
            <p class="text-body1">
              Feed URL: <a :href="feedUrl" target="_blank">{{ feedUrl }}</a>
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
