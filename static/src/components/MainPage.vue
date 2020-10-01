<template>
  <div id="mid">
    <b-alert v-if="showSubmissionAlert" variant="success" dismissible>Success Alert</b-alert>
    <h1>Welcome to <span style="color: #5658dd;">Biome</span></h1>
    <p>A vanilla, whitelisted, Australian-based Minecraft server with a focus on community engagement and
      interaction.</p>
    <b-button class="button-primary custom-button"
              v-b-modal:register-modal>Apply for whitelist</b-button>
    <b-button
        class="button-dark custom-button dark-button">
      <a href="https://discord.gg/YhgEsdZ">Join us on Discord</a>
    </b-button>
    <b-button
        class="button-dark custom-button dark-button">
      <a href="https://biome.pw/map/">View the server map</a>
    </b-button>
    <RegisterModal></RegisterModal>
  </div>
</template>

<script>
import RegisterModal from "@/components/RegisterModal";

export default {
  name: "MainPage",
  components: {RegisterModal},
  data: () => {
    return {
      showSubmissionAlert: false,
    }
  },
}
</script>

<style scoped>
#mid {
  position: relative;
  padding-top: 20%;
  align-items: center;
}

h1 {
  color: #eceded;
}

p {
  color: #9ca9b3;
}

a {
  color: #eceded;
  background-color: inherit;
  text-decoration: none !important;
  text-align: center;
}

.button-primary {
  color: #eceded;
  background-color: #6163ff;
}

.button-dark {
  background-color: #2e3137;
}

.custom-button {
  display: inline-flex;
  font-size: 16px;
  line-height: 24px;
  font-weight: 600;
  padding: 11px 31px;
  height: 48px;
  text-decoration: none !important;
  border: 1px solid transparent;
  border-radius: 2px;
  cursor: pointer;
  justify-content: center;
  text-align: center;
  letter-spacing: inherit;
  white-space: nowrap;
  transition: background .15s ease;
}

.custom-button:hover {
  background-color: #7072ff;
  transition: background .3s ease;
}

.dark-button:hover {
  background-color: #25282c;
  transition: background .3s ease;
}
</style>
