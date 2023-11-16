<template>
  <main
    class="flex flex-col items-center justify-center w-screen h-screen"
    @contextmenu.prevent
  >
    <template v-if="completed">
      <strong class="text-2xl font-bold mb-2"
        >¡Virus informático liberado con éxito!</strong
      >
      <p class="mb-4 text-lg">
        Habéis logrado detener la producción masiva de androides, ¡podéis volver
        al pasado con la victoria en vuestra mano!
      </p>
      <p class="italic">
        Has completado con éxito el Scape Room en
        <span class="text-primary">{{ finalTime }}</span>
      </p>
    </template>
    <template v-else>
      <p class="mb-2 text-lg">
        Procesando la producción en masa de androides NK400-2BW...
      </p>
      <div class="flex flex-row items-center">
        <progress
          class="progress progress-error !w-[30rem]"
          max="100"
          :value="progress"
        ></progress>
        <p class="ml-2">{{ displayProgress }}%</p>
      </div>
      <form class="join mt-4" @submit.prevent="verify">
        <input
          class="input input-secondary join-item w-72"
          placeholder="Esperando el virus informático..."
          v-model="inputCode"
        />
        <button class="btn btn-secondary join-item uppercase" type="submit">
          <span class="loading loading-spinner" v-show="verifying"></span
          >Liberar
        </button>
      </form>
    </template>
  </main>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import intervalToDuration from "date-fns/intervalToDuration";
import formatDuration from "date-fns/formatDuration";
import { es } from "date-fns/locale";
import { invoke } from "@tauri-apps/api/tauri";
import confetti from "canvas-confetti";

const completed = ref(false);

const progress = ref(0);
const displayProgress = computed(() =>
  (Math.floor(progress.value * 100) / 100).toFixed(2)
);
const initialDate = new Date();

setInterval(() => {
  const d = intervalToDuration({
    start: initialDate,
    end: new Date(),
  });
  console.info("d", d);
  progress.value = (100 / 2700) * ((d.minutes ?? 0) * 60 + (d.seconds ?? 0));
}, 500);

const inputCode = ref("");

const verifying = ref(false);
const verify = async () => {
  verifying.value = true;
  await new Promise((r) => setTimeout(r, Math.random() * 2000 + 4000));
  const res = (await invoke("verify", { code: inputCode.value })) as boolean;
  completed.value = res;
  inputCode.value = "";
  verifying.value = false;
};

const finalTime = ref();

watch(completed, (val) => {
  if (!val) return;
  confetti({
    origin: {
      x: 0,
      y: 1,
    },
    angle: 50,
    startVelocity: 85,
    spread: 60,
    particleCount: 70,
    gravity: 0.2,
    ticks: 800,
  });
  true;
  confetti({
    origin: {
      x: 1,
      y: 1,
    },
    angle: 130,
    startVelocity: 85,
    spread: 60,
    particleCount: 70,
    gravity: 0.2,
    ticks: 800,
  });
  finalTime.value = formatDuration(
    intervalToDuration({
      start: initialDate,
      end: new Date(),
    }),
    {
      locale: es,
    }
  );
});
</script>
