<script lang="ts" setup>
import { Menu, PredefinedMenuItem } from '@tauri-apps/api/menu'
import { message } from '@tauri-apps/plugin-dialog'
import {ref, computed, onMounted} from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { Picture as IconPicture } from '@element-plus/icons-vue'

interface Anime {
    title: string,
    description: string,
    staff: Array<[string, string]>,
    cast: Array<[string, string]>,
    cover: string,
}

interface AnimeIndex {
    name: string,
    id: string,
    edit_distance: number
}

onMounted(async () => {
    await invoke('show_main_window')
    netState.value = await invoke('net_is_ok')
    source.value = await invoke('get_id')
})

const netState = ref(false)
const source = ref('获取中……')

const keyWord = ref('')
const tableLoading = ref(false)
const infoLoading = ref(false)
const searchResult = ref<AnimeIndex[]>([])
const resultText = computed(() => {
    return `共计 ${searchResult.value.length} 条检索结果`
})
const SearchAnimeNames = async () => {
    if (keyWord.value) {
        tableLoading.value = true
        searchResult.value = await invoke('get_anime_index', {keyWord: keyWord.value})
        tableLoading.value = false
    } else {
        await message('关键词不能为空！', { title: '提醒！', kind: 'warning' })
    }
    
}

const anime = ref<Anime | null>(null)
const handleTableClick = async (newItem: AnimeIndex, _: AnimeIndex ) => {
    if (newItem) {
        infoLoading.value = true
        anime.value = await invoke('get_anime_info', {animeIndex: newItem})
        infoLoading.value = false
    }
}

document.oncontextmenu = async (e) => {
    let element = e.target as HTMLElement
    const menuItems: PredefinedMenuItem[] = []
    menuItems.push(await PredefinedMenuItem.new({ item: 'Copy', text: "复制" }))
    if (element.tagName === 'INPUT') {
        menuItems.unshift(await PredefinedMenuItem.new({ item: 'Cut', text: "剪切" }))
        menuItems.push(await PredefinedMenuItem.new({ item: 'Paste', text: "粘贴" }))
        menuItems.push(await PredefinedMenuItem.new({ item: 'SelectAll', text: "全选" }))
    } 
    
    const menu = await Menu.new({
        items: menuItems,
    })
    await menu.popup()
}
</script>

<template>
    <div class="container">
        <el-row :gutter="10">
            <el-col :span="6" class="subcontainer">
                <el-row>
                    <el-col :span="12">
                        <el-text :type="netState?'success':'danger'">{{ '网路状态：' + (netState?'已连接':'未连接') }}</el-text>
                    </el-col>
                    <el-col :span="12">
                        <el-text type="success">{{ '数据源：' + source }}</el-text>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col class="search-bar">
                        <el-input @keyup.enter="SearchAnimeNames" v-model="keyWord" placeholder="请输入关键词"/>
                        <el-button type="primary" @click="SearchAnimeNames">搜索</el-button>
                    </el-col>
                </el-row>
                <el-row class="table-box">
                    <el-table empty-text="暂无数据" v-loading="tableLoading" :data="searchResult" @current-change="handleTableClick" highlight-current-row border height="100%" size="small">
                        <el-table-column type="index" width="45" label="序号" />
                        <el-table-column prop="name" :label="resultText" />
                    </el-table>
                </el-row>
            </el-col>
            <el-col :span="6" class="subcontainer" v-loading="infoLoading">
                <el-row>
                    <el-col>
                        <el-image :src="anime?.cover" fit="fill" class="cover">
                            <template #error>
                                <div class="image-slot">
                                    <el-icon><icon-picture /></el-icon>
                                </div>
                            </template>
                        </el-image>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-text type="primary" class="title">{{ anime?anime.title:'暂无内容' }}</el-text>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-divider class="divider" content-position="left">内容简介</el-divider>
                    </el-col>
                </el-row>
                <el-row class="description-box">
                    <el-col>
                        <el-text size="large" v-html="anime?.description"></el-text>
                    </el-col>
                </el-row>
            </el-col>
            <el-col :span="12" class="subcontainer info-box" v-loading="infoLoading" >
                <el-row>
                    <el-col>
                        <el-divider class="divider" content-position="left">声优信息</el-divider>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-descriptions :column="2" border size="small">
                            <el-descriptions-item v-for="[cast_key, cast_value], _ in anime?.cast"
                                :key="cast_key" :label="cast_key" width="25%">
                                {{ cast_value }}</el-descriptions-item>
                        </el-descriptions>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-divider class="divider" content-position="left">制作信息</el-divider>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-descriptions :column="1" border size="small">
                            <el-descriptions-item v-for="[staff_key, staff_value], _ in anime?.staff"
                                :key="staff_key" :label="staff_key">
                                {{ staff_value }}</el-descriptions-item>
                        </el-descriptions>
                    </el-col>
                </el-row>
            </el-col>
        </el-row>
        
    </div>
</template>

<style>
::-webkit-scrollbar {
    display: none;
}
body {
    padding: 0;
    margin: 0;
}
.el-button {
    font-family: '微软雅黑';
}
.container {
    padding: 10px;
}
.container .subcontainer {
    height: calc(100vh - 20px);
    display: flex;
    flex-direction: column;
    row-gap: 10px;
}
.subcontainer .search-bar {
    display: flex;
    column-gap: 10px;
}
.table-box {
    flex: 1;
    overflow: hidden;
}
.title {
    font-size: 24px;
}
.divider {
    margin: 10px 0;
}
.info-box {
    overflow: scroll;
}
.description-box {
    flex: 1;
    overflow: scroll;
}
.image-slot {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    aspect-ratio: 1 / 1.45;
    background: var(--el-fill-color-light);
    color: var(--el-text-color-secondary);
    font-size: 30px;
}
.cover {
    width: 100%;
}
</style>