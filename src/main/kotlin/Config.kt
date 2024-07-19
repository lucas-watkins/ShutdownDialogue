package com.lucaspowered.shutdowndialogue

import java.nio.file.Files
import java.nio.file.Paths
import javax.swing.JOptionPane

class Config {
   companion object {
       private val contents = getConfigContents()

       private val hmap = getConfigContents()

       private fun getConfigContents(): HashMap<String, String> {
           val map = HashMap<String, String>()
           if (!Files.exists(Paths.get(configFileName))) {
               Files.createFile(Paths.get(configFileName))
           }
           Files.readAllLines(Paths.get(configFileName)).forEach {
               val split = it.split("=")
               map[split[0].trim()] = split[1].trim()
           }
           return map
       }

       fun get(value: String): String{
           val rtrn = hmap[value] ?: ""
           if (rtrn.isEmpty()) {
               JOptionPane.showMessageDialog(
                   null,
                   "Config File Error, see Github for example.",
                   "Shutdown Dialogue",
                   JOptionPane.ERROR_MESSAGE
               )
           }
           return rtrn
       }

   }
}

