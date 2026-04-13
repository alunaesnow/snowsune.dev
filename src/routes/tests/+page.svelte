<script lang="ts">
	import { fib, vectest } from '$lib/pkg/rs';
	import { memory } from '$lib/pkg/rs_bg.wasm';

	function testArrPtr() {
		let ptr = vectest(2);

		let arr = new Uint8Array(memory.buffer, ptr, 10);
		console.log(ptr, arr);
	}
</script>

<button onclick={testArrPtr}> Test array pointers </button>
