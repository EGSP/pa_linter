<script lang="ts">
    export let display: string|null = null;
    export let direction: string ='row';
</script>

<div class="frame" style='flex-direction:{direction};display:{display};'>
<slot/>
</div>

<style>
    .frame {
        display: flex;
        width: 100%;
        height: 100%;
    }
</style>