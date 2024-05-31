# Inline script

Inline scripts bound to the parent html element

<div>
    <output></output>
    <script>
        (() => {
            let $root = document.currentScript.parentElement;
            let $output = $root.querySelector("output");
            $output.textContent = "hi from the script";
        })();
    </script>
</div>
