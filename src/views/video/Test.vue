<template>
  <div>
    <!-- 添加按钮 -->
    <el-button-group>
      <el-button type="primary" :disabled="!selectedRows.length" @click="startTask">开始任务</el-button>
      <el-button type="warning" :disabled="!selectedRows.length" @click="stopTask">结束任务</el-button>
      <el-button type="success" @click="addTask">添加任务</el-button>
    </el-button-group>

    <!-- 表格 -->
    <el-table
      v-loading="loading"
      :data="tableData"
      @selection-change="onSelectionChange"
    >
      <el-table-column type="selection" width="55"></el-table-column>
      <el-table-column prop="filename" label="文件名"></el-table-column>
      <el-table-column prop="videoEncoding" label="视频编码"></el-table-column>
      <el-table-column prop="framerate" label="帧率"></el-table-column>
      <el-table-column prop="audioEncoding" label="音频编码"></el-table-column>
      <el-table-column prop="samplingRate" label="采样率"></el-table-column>
      <el-table-column prop="speed" label="倍速"></el-table-column>
      <el-table-column label="转码进度">
        <template #default="{ row }">
          <el-progress :percentage="row.progress" :color="getProgressColor(row)"></el-progress>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script>
import { ref } from "vue";
import { ElButton, ElButtonGroup, ElTable, ElTableColumn, ElProgress } from "element-plus";

export default {
  components: {
    ElButton,
    ElButtonGroup,
    ElTable,
    ElTableColumn,
    ElProgress
  },
  setup() {
    // 初始化数据
    const loading = ref(false);
    const selectedRows = ref([]);
    const tableData = ref([
      { id: 1, filename: 'file1.mp4', videoEncoding: 'H.264', framerate: 30, audioEncoding: 'AAC', samplingRate: 48000, speed: 1.0, progress: 50 },
      { id: 2, filename: 'file2.mp4', videoEncoding: 'H.265', framerate: 24, audioEncoding: 'MP3', samplingRate: 44100, speed: 1.5, progress: 80 },
      { id: 3, filename: 'file3.mp4', videoEncoding: 'MPEG-4', framerate: 60, audioEncoding: 'FLAC', samplingRate: 96000, speed: 2.0, progress: 100 },
    ]);

    // 处理表格选中事件
    const onSelectionChange = (selection) => {
      selectedRows.value = selection;
    };
    // 获取进度条颜色
    const getProgressColor = (row) => {
      if (row.progress >= 100) {
        return '#67c23a';
      } else if (row.progress > 50) {
        return '#f90';
      } else {
        return '#909399';
      }
    };
    // 开始任务
    const startTask = () => {
      console.log('开始任务', selectedRows.value);
    };
    // 结束任务
    const stopTask = () => {
      console.log('结束任务', selectedRows.value);
    };
    // 添加任务
    const addTask = () => {
      console.log('添加任务');
    };

    return {
      loading,
      selectedRows,
      tableData,
      onSelectionChange,
      getProgressColor,
      startTask,
      stopTask,
      addTask
    };
  }
};
</script>
