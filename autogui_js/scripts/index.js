let adb = new Adb("127.0.0.1:5037", "127.0.0.1:7555", "adb.exe")

function OnGameStart() {
    let img = adb.screenshot()
    console.log("start")
    let templ = Image.open("./templ.png")
    let point = img.match_template(templ)
    if (point == null) {
        throw "can not find templ"
    }
    adb.click(point.x + templ.width() / 2, point.y + templ.height() / 2)
    console.log("end")
}

OnGameStart()