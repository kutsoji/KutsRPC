/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        './*.html',
        './src/*.rs',
        './**/*.rs'],
    theme: {
        colors: {
            dc_black: "#202225",
            dc_nav: "#36393F",
            dc_gray: "#B9BBBE",
            dc_hover: "#42464D",
            dc_textbar: "#40444B",
            dc_blue: "#5865F2",
            dc_green: "#3BA55D",
            dc_white: "#DCDDDE",
            dc_btn: "#41434A",
            black: "#000000"
        },
        fontFamily: {
            'code': ['Fira Code', 'monospace'],
        },
        extend: {
            gridTemplateColumns: {
                '14': 'repeat(14, minmax(0, 1fr))',
            }
        },
    },
    plugins: [],
}