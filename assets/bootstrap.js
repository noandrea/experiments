import('./factorial_calculator').then(module => {
    window.calculate = () => {
        const input = document.getElementById('numberInput').value;
        // const result = module.factorial(parseInt(input));
        const result = module.make_curve(4,14,80,50,100);
        document.getElementById('result').innerText = result;
    };
}).catch(console.error);