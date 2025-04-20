package ix.radon.rustydroid

public object NativeApi {
    init {
        System.loadLibrary("demo")
    }

    public external fun increment(i: Int)
}
