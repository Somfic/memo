<script lang="ts">
    import Cards from "./Cards.svelte";

    let previous: () => void;
    let advance: () => void;
</script>

<div class="content-wrapper">
    <div class="content">
        <button on:click={previous} class="previous-button">Previous</button>
        <Cards
            words={[
                "A",
                "B",
                "C",
                "D",
                "E",
                "F",
                "G",
                "H",
                "I",
                "J",
                "K",
                "L",
                "M",
                "N",
                "O",
                "P",
                "Q",
                "R",
                "S",
                "T",
                "U",
                "V",
                "W",
                "X",
                "Y",
                "Z",
            ]}
            bind:previous
            bind:advance
        />
        <button on:click={advance} class="advance-button">Next</button>
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

    .advance-button {
        margin-top: 20px;
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 5px;
        transition: background-color 0.3s ease;
    }

    .advance-button:hover {
        background-color: #0056b3;
    }

    .previous-button {
        margin-top: 20px;
        margin-right: 10px;
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
        background-color: #6c757d;
        color: white;
        border: none;
        border-radius: 5px;
        transition: background-color 0.3s ease;
    }

    .previous-button:hover {
        background-color: #5a6268;
    }
</style>
