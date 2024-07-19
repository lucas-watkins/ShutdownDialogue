package com.lucaspowered.shutdowndialogue

import java.awt.FlowLayout
import java.awt.event.ActionEvent
import java.awt.event.ActionListener
import javax.swing.*

const val configFileName = "config"

class Window : ActionListener {

    private val frame = JFrame()
    private val lockBtn = JButton("Lock")
    private val logoutBtn = JButton("Logout")
    private val shutdownBtn = JButton("Shutdown")

    init {
        frame.setSize(300, 100)
        frame.defaultCloseOperation = JFrame.EXIT_ON_CLOSE
        frame.layout = FlowLayout(FlowLayout.CENTER, 10, 10)
        frame.title = "Shutdown Dialogue"
        frame.isResizable = false

        for (ele in arrayOf(lockBtn, logoutBtn, shutdownBtn)) {
            frame.add(ele)
        }

        frame.isVisible = true
    }

    override fun actionPerformed(e: ActionEvent) {
      when (e.source) {
           lockBtn -> runProcess("")
      }
    }

    private fun runProcess(p: String){
        val builder = ProcessBuilder()
        builder.command(p.split(" "))
        builder.start()
    }
}

fun main(){
    Window()
}