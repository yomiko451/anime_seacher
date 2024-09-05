<template>
    <div class="contextmenu" :style="{top: pTop + 'px', left: pLeft + 'px' }" v-show="isShow" id="contextmenu">
        <div @click="cutText" @mousedown="getNoFocus">剪切</div>
        <div @click="copyText" @mousedown="getNoFocus">复制</div>
        <div @click="pasteText" @mousedown="getNoFocus">粘贴</div>
        <div @click="selectAll" @mousedown="getNoFocus">全选</div>
        <div @click="deleteText" @mousedown="getNoFocus">删除</div>
    </div>
</template>

<script lang="ts" setup>
import {ref} from 'vue'
import {readText, writeText} from '@tauri-apps/plugin-clipboard-manager'


const pLeft = ref(0)
const pTop = ref(0)
const isShow = ref<boolean>(false)
document.addEventListener('click', (e)=>{
    if (e.button === 0) {
        isShow.value=false
    }
})
document.oncontextmenu = (e) => {
    e.preventDefault()
    let element = e.target as HTMLElement
    if (element.tagName === 'INPUT') {
        const { clientX, clientY } = e;
        pLeft.value = clientX
        pTop.value = clientY
        isShow.value = true
    }
}

async function copyText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart !== null && selectelement.selectionEnd !== null) {
        const text = selectelement.value.substring(selectelement.selectionStart, selectelement.selectionEnd)
        await writeText(text)
    }
}

async function cutText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const text = selectelement.value.substring(selectelement.selectionStart, selectelement.selectionEnd)
        await writeText(text)
        const temp = selectelement.selectionStart
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + selectelement.value.substring(selectelement.selectionEnd)
        selectelement.selectionStart = selectelement.selectionEnd = temp
        selectelement.dispatchEvent(new Event('input'))
    }
}

async function pasteText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const text = await readText()
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + text + selectelement.value.substring(selectelement.selectionEnd)
        selectelement.dispatchEvent(new Event('input'))
    }
}

function selectAll() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        selectelement.selectionStart = 0
        selectelement.selectionEnd = selectelement.value.length
    }
}

function deleteText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const temp = selectelement.selectionStart
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + selectelement.value.substring(selectelement.selectionEnd)
        selectelement.selectionStart = selectelement.selectionEnd = temp
        selectelement.dispatchEvent(new Event('input'))
    }
}

function getNoFocus(e: MouseEvent) {
    e.preventDefault()
}
</script>

<style scoped>
.contextmenu {
    position: fixed;
    display: flex;
    row-gap: 4px;
    padding: 4px;
    flex-direction: column;
    width: 65px;
    background-color: white;
    color: var(--el-text-color-secondary);
    box-shadow: var(--el-box-shadow);
    z-index: 10;
    border: 1px solid var(--el-border-color);
}
.contextmenu>div {
    font-size: 14px;
    padding: 0 0.5rem;
    transition: all 0.1s;
    user-select: none;
}
.contextmenu>div:hover {
    background-color: #409EFF;
    color: white;
}
</style>