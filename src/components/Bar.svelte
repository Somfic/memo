<div class="bar">
    <div class="items">
        <div class="item left">left</div>
        <div class="item center">center</div>
        <div class="item right">right</div>
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
