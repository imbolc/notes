# Dialog html tag

- `onclick="event.target == this && this.close()"` - closes the dialog  when clicking the backdrop
- `html:has(dialog[open]) { overflow: hidden }` - disables page scrolling when dialog is shown
- `autofocus` - is usually better to have on the `Cancel` button to prevent accidental submitting



## Example
<style>
    html:has(dialog[open]) {
        overflow: hidden;
    }
    dialog {
        padding: 0;
    }
</style>
<dialog id="d" onclick="event.target == this && this.close()">
    <div>
        <header>
            <h2>Dialog</h2>
        </header>
        <article>
            <p>Hey :)</p>
        </article>
        <footer>
            <button type="submit" value="confirm">Confirm</button>
            <button autofocus type="reset" onclick="this.closest('dialog').close(); return false">Cancel</button>
        </footer>
    </div>
</dialog>
<button onclick="d.showModal()">Open Dialog</button>

