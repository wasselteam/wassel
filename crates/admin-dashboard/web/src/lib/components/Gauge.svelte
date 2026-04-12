<script module lang="ts">
  export type GaugeProps = {
    ratio: number;
    text: string;
  };
</script>

<script lang="ts">
  let { ratio, text }: GaugeProps = $props();
  let arcStyle = $derived(`--a: ${ratio * 360}deg`);
</script>

<div class="card">
  <span class="text">{text}<br />{Math.round(ratio * 1000) / 10}%</span>
  <div class="circle-bg"></div>
  <div class="circle" style={arcStyle}></div>
</div>

<style>
  .card {
    width: 100%;
    height: 150px;
    padding: 7px;
    background: #f4f4f4;
    text-align: center;
    box-shadow:
      inset 1px 1px 2px #cfcfcf,
      inset -1px -1px 2px #cfcfcf;
  }

  span.text {
    position: absolute;
    left: 50%;
    top: 50%;
    translate: -50% -50%;
    font-size: 0.8em;
    line-height: 1.15em;
  }

  div.circle-bg,
  div.circle {
    --b: 8px;
    width: 134px;
    aspect-ratio: 1;
    padding: var(--b);
    box-sizing: border-box;
    border-radius: 50%;
    position: absolute;
    left: 50%;
    translate: -50%;
  }

  div.circle-bg {
    background: #ddd;
    mask:
      linear-gradient(#0000 0 0) content-box intersect,
      conic-gradient(#000 360deg);
    -webkit-mask:
      linear-gradient(#0000 0 0) content-box intersect,
      conic-gradient(#000 360deg);
  }

  div.circle {
    --a: 220deg;
    rotate: 180deg;
    background: var(--color-primary-dark);
    --_g: /var(--b) var(--b) no-repeat radial-gradient(50% 50%, #000 calc(100% - 1px), #0000);
    mask:
      top var(--_g),
      calc(50% + 50% * sin(var(--a))) calc(50% - 50% * cos(var(--a))) var(--_g),
      linear-gradient(#0000 0 0) content-box intersect,
      conic-gradient(#000 var(--a), #0000 0);
  }

  .card {
    position: relative;
  }
</style>
