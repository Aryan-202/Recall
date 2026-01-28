package com.brain.dumps.recall.home

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview


@Composable
fun SideBar() {
    Column(
        modifier = Modifier
            .fillMaxHeight()

    ) {
        Text(text = "Recall")
        Button(onClick = { /*TODO*/ }) {
            Text(text = "New Note")
        }
    }
}

@Preview
@Composable
fun SideBarPreview() {
    SideBar()
}
