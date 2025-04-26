<script lang="ts">
  import { commands, type AnswerQuality, type Card } from "$gen/bindings";
  import { onMount } from "svelte";
  import { card } from "../state";

  export let state: "Thinking" | "WasRight" | "WasWrong" = "Thinking";

  let previews: Partial<{ [key in AnswerQuality]: string }> | undefined;

  card.subscribe(async (card) => {
    previews = await commands.previewNextReview(card);
  });
</script>

<div class="bar">
  <div class="items">
    <div class="item left">{previews ? previews["OkEasy"] : ""}</div>
    <div class="item center">{previews ? previews["OkHesitated"] : ""}</div>
    <div class="item right">{previews ? previews["OkDifficult"] : ""}</div>
  </div>
</div>

<style lang="scss">
  .bar {
    background-color: black;
    padding: 15px;
    border-radius: 40px;
    z-index: 100;
  }

  .bar,
  .items {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .items {
    gap: 3px;
  }

  .item {
    background-color: #1a1a1a;
    color: white;
    padding: 10px 22px;
    border-radius: 10px;

    &.left {
      border-top-left-radius: 20px;
      border-bottom-left-radius: 20px;
    }

    &.right {
      border-top-right-radius: 20px;
      border-bottom-right-radius: 20px;
    }
  }
</style>
