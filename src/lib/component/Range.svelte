<script lang="ts">
    import {onMount} from "svelte";

    export let thumb = "rgb(37, 99, 235)";
    export let background = "#35394d";
    /** should be thumb under almost any circumstance */
    export let track = thumb;
    export let value = 0;
    export let min = 0;
    export let max = 1;
    export let step = 0.1;
    export let textbox: false | string = false;
    const initial = value;
    const onUnfocus = () => {
        if (value % step !== 0) value = Math.round(value / step) * step;
        if (isNaN(value)) value = initial;
        if (value < min) value = min;
        if (value > max) value = max;
    };
    export let styles: Partial<{
        rangeContainer: string;
        range: string;
        rangeProgressOverlay: string;
        input: string;
        textbox: string;
    }> = {
        range: "/*blank*/",
        rangeContainer: "/*blank*/",
        rangeProgressOverlay: "/*blank*/",
        textbox: "/*blank*/",
        input: "/*blank*/",
    };
    $: progress = ((value - min) / (max - min)) * 100;
    let isLinux = false;
    onMount(() => {
        isLinux = navigator.userAgent.includes("Linux");
    });
</script>

{@html `<!-- value: ${typeof value === "number" ? value : "no-xss-pls-ty"} -->`}
<div
        class="range"
        style="--bg:{background};--thumb:{thumb};--track:{track};{styles.rangeContainer}"
>
    <input
            class="bg-slate-800"
            type="range"
            {min}
            {max}
            {step}
            bind:value
            on:blur={onUnfocus}
            style={styles.range}
    />
    <div
            class="trackProgressOverlay"
            style="--progress:{progress}%;--added:{progress < 15
			? '14px'
			: progress < 33
			? '8px'
			: '0px'};{styles.rangeProgressOverlay};--additionalTop:{isLinux
			? '-1px'
			: '0px'};"
    />
</div>
{#if textbox}
    <div class="input mt-1" style={styles.input}>
        {textbox.split("$input$")[0] ?? ""}<input
            type="number"
            bind:value
            {step}
            {min}
            {max}
            style="width:{value.toString().length}ch;{styles.textbox}"
            on:blur={onUnfocus}
    />
        <span style="font-size: 0.8rem">{textbox.split("$input$")[1] ?? ""}</span>
    </div>
{/if}

<style lang="scss">
  $size: 8px;
  $thumbSize: 18px;
  .range {
    width: 100%;
    position: relative;

    input[type="range"] {
      height: $size;
      border-radius: 25px;
      width: 100%;
      -webkit-appearance: none;
      appearance: none;
      outline: none;
      @mixin thumb {
        @apply drop-shadow-lg;
        height: $thumbSize;
        width: $thumbSize;
        background: var(--thumb, #fff);
        cursor: pointer;
        outline: none;
        border: none;
        border-radius: 100%;
        position: relative;
        z-index: 5;
        transition: border 0.2s;
        border: 4px solid #0000;
        &:hover,
        &:focus,
        &:active {
          border: 4px solid var(--thumb, #fff);
        }
      }

      &::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        @include thumb;
      }

      &::-moz-range-thumb {
        @include thumb;
      }

      &::-ms-thumb {
        @include thumb;
      }
    }

    .trackProgressOverlay {
      position: absolute;
      top: calc(
              #{18 + (max($size - 18, 0)) - $size} + var(--additionalTop, 0px)
      );
      left: 0;
      height: $size;
      width: calc(var(--progress) + var(--added, 0px));
      min-width: $thumbSize;
      background: var(--track, var(--thumb, #fff));
      border-radius: 25px;
      pointer-events: none;
    }
  }

  .input {
    display: flex;

    > input[type="number"] {
      border: none;
      background: transparent;
      color: inherit;
      font-size: 0.8rem;
      font-family: inherit;
      text-align: inherit;
      padding: 0;
      margin: 0;
      // appearance stuff:
      appearance: textfield;
      -moz-appearance: textfield;

      &::-webkit-outer-spin-button,
      &::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
      }
    }
  }
</style>
