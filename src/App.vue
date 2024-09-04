<script lang="ts" setup>
import {ref, computed} from 'vue'
import { invoke } from '@tauri-apps/api/core';

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

const keyWord = ref('')
const imgUrl = ref('https://lain.bgm.tv/r/400/pic/cover/l/2c/15/184840_1kbri.jpg')

const searchResult = ref<AnimeIndex[]>([])
const resultText = computed(() => {
    return `共计 ${searchResult.value.length} 条检索结果`
})
const SearchAnimeNames = async () => {
    searchResult.value = await invoke('get_list', {keyWord: keyWord.value})
}

//TODO:完成表格点击事件
const anime = ref<Anime | null>(null)
const handleTableClick = async (newItem: AnimeIndex, _: AnimeIndex ) => {
    anime.value = await invoke('get_info', {animeIndex: newItem})
}
</script>

<template>
    <div class="container">
        <el-row :gutter="10">
            <el-col :span="6" class="subcontainer">
                <el-row>
                    <el-col :span="12">
                        <el-text type="success">网路状态：已连结</el-text>
                    </el-col>
                    <el-col :span="12">
                        <el-text type="success">数据源：bangumi</el-text>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col class="search-bar">
                        <el-input v-model="keyWord" placeholder="请输入关键词"/>
                        <el-button type="primary" @click="SearchAnimeNames">搜索</el-button>
                    </el-col>
                </el-row>
                <el-row class="table-box">
                    <el-table :data="searchResult" @current-change="handleTableClick" highlight-current-row border height="100%" size="small">
                        <el-table-column type="index" width="45" label="序号" />
                        <el-table-column prop="name" :label="resultText" />
                    </el-table>
                </el-row>
            </el-col>
            <el-col :span="6" class="subcontainer">
                <el-row>
                    <el-col>
                        <el-image :src="anime?anime.cover:imgUrl" fit="fill"/>
                    </el-col>
                </el-row>
                  
            </el-col>
            <el-col :span="12" class="subcontainer">
                <el-row>
                    <el-col>
                        <el-text type="primary" class="title">{{ anime?anime.title:'暂无标题' }}</el-text>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-divider class="divider" content-position="left">内容简介</el-divider>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-text size="large">{{ anime?anime.description:'暂无内容' }}</el-text>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-divider class="divider" content-position="left">声优信息</el-divider>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col>
                        <el-divider class="divider" content-position="left">制作信息</el-divider>
                    </el-col>
                </el-row>
            </el-col>
        </el-row>
    </div>
</template>

<style>
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
</style>