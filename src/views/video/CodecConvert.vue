<template>
  <div class="container">
    <el-form label-position="top">
      <el-form-item label="输入文件">
        <el-upload :on-success="handleUploadSuccess"
          :show-file-list="false" :before-upload="beforeUpload">
          <el-button size="small" type="primary">点击上传</el-button>
          <div slot="tip" class="el-upload__tip">支持 mp4、avi、mov 等常见视频格式</div>
        </el-upload>
      </el-form-item>
      <el-form-item label="输出文件格式">
        <el-radio-group v-model="outputFormat">
          <el-radio-button label="mp4">MP4</el-radio-button>
          <el-radio-button label="avi">AVI</el-radio-button>
          <el-radio-button label="mov">MOV</el-radio-button>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="视频编解码器">
        <el-select v-model="videoCodec" placeholder="请选择视频编解码器">
          <el-option label="H.264" value="libx264"></el-option>
          <el-option label="H.265/HEVC" value="libx265"></el-option>
          <el-option label="VP8" value="libvpx"></el-option>
          <el-option label="VP9" value="libvpx-vp9"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="音频编解码器">
        <el-select v-model="audioCodec" placeholder="请选择音频编解码器">
          <el-option label="AAC" value="aac"></el-option>
          <el-option label="MP3" value="libmp3lame"></el-option>
          <el-option label="Opus" value="opus"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="比特率">
        <el-input-number v-model="bitrate" controls-position="right" :min="0" :step="100" :precision="0" :max="20000"
          placeholder="请输入比特率"></el-input-number>
      </el-form-item>
      <el-form-item label="分辨率">
        <el-row>
          <el-col :span="11">
            <el-input v-model="width" placeholder="宽度"></el-input>
          </el-col>
          <el-col :span="2">
            <div class="text-center">x</div>
          </el-col>
          <el-col :span="11">
            <el-input v-model="height" placeholder="高度"></el-input>
          </el-col>
        </el-row>
      </el-form-item>
      <el-form-item label="帧率">
        <el-input-number v-model="framerate" controls-position="right" :min="1" :step="1" :precision="1" :max="240"
          placeholder="请输入帧率"></el-input-number>
      </el-form-item>
      <el-form-item label="播放速度">
        <el-input-number v-model="speed" controls-position="right" :min="0.1" :step="0.1" :precision="1" :max="10"
          placeholder="请输入播放速度"></el-input-number>
      </el-form-item>
      <el-form-item label="裁剪">
        <el-row>
          <el-col :span="10">
            <el-input v-model="cropLeft" placeholder="左边距"></el-input>
          </el-col>
          <el-col :span="2">
            <div class="text-center">-</div>
          </el-col>
          <el-col :span="10">
            <el-input v-model="cropRight" placeholder="右边距"></el-input>
          </el-col>
        </el-row>
        <el-row style="margin-top:5px">
          <el-col :span="10">
            <el-input v-model="cropTop" placeholder="上边距"></el-input>
          </el-col>
          <el-col :span="2">
            <div class="text-center">-</div>
          </el-col>
          <el-col :span="10">
            <el-input v-model="cropBottom" placeholder="下边距"></el-input>
          </el-col>
        </el-row>
      </el-form-item>
      <el-form-item label="添加水印">
        <el-switch v-model="watermark">
        </el-switch>
      </el-form-item>
      <el-form-item label="水印位置">
        <el-select v-model="watermarkPosition" placeholder="请选择水印位置">
          <el-option label="左上" value="10:10"></el-option>
          <el-option label="右上" value="(W-w-10):10"></el-option>
          <el-option label="左下" value="10:(H-h-10)"></el-option>
          <el-option label="右下" value="(W-w-10):(H-h-10)"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="convertVideo">开始转换</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script lang="ts">
export default {
  data() {
    return {
      token: '',
      uploadUrl: '',
      inputFilePath: '',
      outputFormat: 'mp4',
      videoCodec: 'libx264',
      audioCodec: 'aac',
      bitrate: 4000,
      width: '',
      height: '',
      framerate: 30,
      speed: 1.0,
      cropLeft: '',
      cropRight: '',
      cropTop: '',
      cropBottom: '',
      watermark: false,
      watermarkPosition: '',
    }
  },
  methods: {
    beforeUpload(file) {
      this.inputFilePath = URL.createObjectURL(file)
    },
    handleUploadSuccess(response, file) {
      this.uploadUrl = response.url
      this.token = response.token
      this.message.success('上传成功')
    },
    convertVideo() {
      // 检查输入文件是否正确
      if (!this.inputFilePath) {
        this.message.error('请先上传视频文件')
        return
      }

      // 检查输出分辨率是否正确
      if (this.width % 2 === 1 || this.height % 2 === 1) {
        this.$message.warning('输出分辨率必须是偶数')
        return
      }

      // 构建命令
      let command = `ffmpeg -i ${this.inputFilePath} -c:v ${this.videoCodec} -c:a ${this.audioCodec} `
      if (this.bitrate) {
        command += `-b:v ${this.bitrate}k `
      }
      if (this.width && this.height) {
        command += `-s ${this.width}x${this.height} `
      }
      if (this.framerate) {
        command += `-r ${this.framerate} `
      }
      if (this.speed !== 1.0) {
        command += `-filter:v "setpts=${1 / this.speed}*PTS" `
      }
      if (this.cropLeft || this.cropRight || this.cropTop || this.cropBottom) {
        const left = this.cropLeft || '0'
        const right = this.cropRight || '0'
        const top = this.cropTop || '0'
        const bottom = this.cropBottom || '0'
        command += `-filter:v "crop=in_w-${left}-${right}:in_h-${top}-${bottom}" `
      }
      if (this.watermark) {
        if (this.watermarkPosition) {
          command += `-i watermark.png -filter_complex "overlay=${this.watermarkPosition}" `
        } else {
          this.$message.warning('请选择水印位置')
          return
        }
      }
      command += `-y output.${this.outputFormat}`
      console.log(command)
      // 执行命令
      // this.$axios.post('/api/execute', { command }).then(res => {
      //   if (res.data.success) {
      //     this.$message.success('转换成功')
      //   } else {
      //     this.$message.error(`转换失败：${res.data.message}`)
      //   }
      // }).catch(err => {
      //   this.$message.error(`转换失败：${err.message}`)
      // })
    },
  },
}
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding-top: 20px;
  box-sizing: border-box;
}

.text-center {
  text-align: center;
  margin-top: 7px;
}
</style>