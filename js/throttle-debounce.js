// Throttle and debounce functions

const throttled = throttle((n) => console.log(new Date(), "throttled", n));
throttled(1);
throttled(2);
throttled(3);

const debounced = debounce((n) => console.log(new Date(), "debounced", n));
debounced(1);
debounced(2);
debounced(3);

// Ensures the function executed at most once in the `delay`.
// This version ensures the last call.
// It usually used for scroll or resize events
function throttle(func, delay = 250) {
	let called_at = 0;
	let timeout;
	return (...args) => {
		const now = new Date();
		const left = delay - (now - called_at);
		clearTimeout(timeout);
		if (left <= 0) {
			func(...args);
			called_at = now;
		} else {
			timeout = setTimeout(() => {
				func(...args);
				called_at = now;
			}, left);
		}
	};
}

// Runs the function after event stops repeating for `delay`.
// Most suitable for rate-limiting api requests (e.g. for autocompletion)
function debounce(func, delay = 250) {
	let timeout;
	return (...args) => {
		clearTimeout(timeout);
		timeout = setTimeout(() => {
			func(...args);
		}, delay);
	};
}
