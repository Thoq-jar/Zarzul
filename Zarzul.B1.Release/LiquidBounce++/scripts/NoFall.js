var scriptName = "Nofall";
var scriptVersion = 1.0;
var scriptAuthor = "Minger";
var NofallModule = moduleManager.getModule("NOFALL");

var Nofall= new Nofall();

var client;

function Nofall() {
    this.getName = function() {
        return "Nofall";
    };

    this.getDescription = function() {
        return "";
    };

    this.getCategory = function() {
        return "Fun";
    };
    this.onEnable = function() {
		NofallModule.setState(false)
    }
    this.onUpdate = function() {
		mc.thePlayer.setSprinting(true)
        if (mc.gameSettings.keyBindJump.isKeyDown() && (mc.thePlayer.fallDistance > 5)) {

			NofallModule.setState(true)

}else{
		NofallModule.setState(false)
		}
}
    this.onDisable = function () {
        NofallModule.setState(true)
    }
}

function onLoad() {}

function onEnable() {
    client = moduleManager.registerModule(NofallPlus);
}

function onDisable() {
    moduleManager.unregisterModule(client);
}