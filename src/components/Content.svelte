<script>
    import Cards from "./Cards.svelte";
</script>

<div class="content-wrapper">
    <div class="content">
        <Cards words={["A", "B", "C", "D", "E", "F", "G"]} />
    </div>
</div>

<style lang="scss">
    .content-wrapper {
        flex-grow: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .content {
        display: flex;
        align-items: center;
        justify-content: center;
        max-width: 800px;
        max-height: 400px;
        width: 100%;
        height: 100%;
    }
</style>
