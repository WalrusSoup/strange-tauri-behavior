<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  
  async function closeWindowToSimulateCrash() {
    await invoke("close_window"); 
  }
</script>

<template>
  <div class="container">
    <h1>Crash My Window</h1>

    <div class="row">
        <a href="#" @click="closeWindowToSimulateCrash">Close the main window</a>
    </div>
  
  </div>
</template>