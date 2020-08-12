let fs = require('fs');

let main = async () => {
    console.log('early return');
    return;
    try {
        let separator = "// >>>>>>>";
        let devJs = fs.readFileSync('filmow-dev.user.js').toString().split(separator)[0];
        let newJsCode = "console.log('AAAAAA');";

        fs.writeFileSync('filmow-dev.user.js', devJs + separator + "\n\n" + newJsCode);

        console.log("Ok");
    } catch (err) {
        console.error("Error:", err);
    }
}

main().then();
