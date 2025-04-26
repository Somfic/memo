<script lang="ts">
    export let time: string | undefined = undefined;

    function formatTime(time: string | undefined): string {
        if (!time) {
            return "unknown";
        }

        let now = new Date();
        let date = new Date(time);
        let diff = date.getTime() - now.getTime();
        let days = Math.floor(diff / (1000 * 60 * 60 * 24));

        switch (true) {
            case days <= 0:
                return "today";
            case days === 1:
                return "tomorrow";
            case days < 7:
                return `in ${days} days`;
            case days < 14:
                return "next week";
            case days < 30:
                return "next month";
            case days < 365:
                return `in ${Math.floor(days / 30)} months`;
            default:
                return "next year";
        }
    }
</script>

<span>{formatTime(time)}</span>
