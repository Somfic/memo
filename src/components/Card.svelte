<script lang="ts">
    export let word: string;
    export let isInBackground: boolean;
</script>

<div class="card" class:is-in-background={isInBackground}>
    {word}
</div>

<style lang="scss">
    .card {
        background-color: #f0f0f0;
        border-radius: 30px;
        width: 364px;
        height: 167px;
        font-size: 30px;
        font-weight: bold;
        box-shadow: 0 0 40px rgba(0, 0, 0, 0.1);

        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.3s ease-in-out;

        &.is-in-background {
            color: transparent;
        }
    }
</style>
