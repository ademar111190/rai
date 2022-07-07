package ademar.rai

import ademar.rai.lib.Rai
import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class MainActivity : AppCompatActivity() {

    companion object {
        init {
            System.loadLibrary("rai")
        }
    }

    private val scope = CoroutineScope(Dispatchers.Main)
    private val txt: TextView
        get() = findViewById(R.id.txt)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        scope.launch {
            txt.text = communicateToRust()
        }
    }

    private suspend fun communicateToRust(): String = withContext(Dispatchers.IO) {
        Rai.helloWorldSync("Android")
    }

}
