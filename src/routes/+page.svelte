<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMount } from "svelte";

let message = "";
onMount(()=>{
  invoke("greet", {name: "Recall"}).then((res)=>{
    message = res as string
  })
})
</script>

<main>
hello 
{message}
</main>

<style>

</style>