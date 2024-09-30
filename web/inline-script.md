# Inline script

Inline scripts bound to the parent html element

<div>
    <output></output>
    <script>
        (() => {
            let $parent = document.currentScript.parentElement;
            let $output = $parent.querySelector("output");
            $output.textContent = "hi from the script";
        })();
    </script>
</div>
