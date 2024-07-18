package com.lucaspowered.shutdowndialogue

import java.awt.FlowLayout
import java.awt.event.ActionEvent
import java.awt.event.ActionListener
import javax.swing.*


class Window : ActionListener {

    private val frame = JFrame()
    private val lockBtn = JButton("Lock")
    private val logoutBtn = JButton("Logout")
    private val shutdownBtn = JButton("Shutdown")

    init {
        frame.setSize(500, 350)
        frame.defaultCloseOperation = JFrame.EXIT_ON_CLOSE
        frame.layout = FlowLayout(FlowLayout.LEFT, 10, 10)
        frame.title = "Shutdown Dialogue"
        frame.add(lockBtn)
        frame.add(logoutBtn)
        frame.add(shutdownBtn)
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