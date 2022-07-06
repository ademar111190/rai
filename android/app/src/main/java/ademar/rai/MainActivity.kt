package ademar.rai

import ademar.rai.lib.Rai
import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    companion object {
        init {
            System.loadLibrary("rai")
        }
    }

    private val txt: TextView
        get() = findViewById(R.id.txt)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        Thread {
            val result = Rai.helloWorldSync("Android")
            runOnUiThread {
                txt.text = result
            }
        }.start()
    }

}
