<template>
  <div>
    <h1>FFmpeg参数选择器</h1>
    <el-form ref="form" label-position="top">
      <el-collapse v-model="activeNames">
        <el-collapse-item title="视频">
          <el-row :gutter="20">
            <el-col :span="8">
              <el-form-item label="格式">
                <el-select v-model="video.format">
                  <el-option v-for="option in videoFormatOptions" :key="option.value" :label="option.label"
                    :value="option.value"></el-option>
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="编码器">
                <el-select v-model="video.codec">
                  <el-option v-for="option in videoCodecOptions" :key="option.value" :label="option.label"
                    :value="option.value"></el-option>
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="帧率">
                <el-slider v-model="video.framerate" :min="1" :max="60" :step="1" :show-input="true"></el-slider>
              </el-form-item>
            </el-col>
          </el-row>
        </el-collapse-item>
        <el-collapse-item title="音频">
          <el-row :gutter="20">
            <el-col :span="8">
              <el-form-item label="格式">
                <el-select v-model="audio.format">
                  <el-option v-for="option in audioFormatOptions" :key="option.value" :label="option.label"
                    :value="option.value"></el-option>
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="采样率">
                <el-select v-model="audio.samplerate">
                  <el-option v-for="option in audioSamplerateOptions" :key="option.value" :label="option.label"
                    :value="option.value"></el-option>
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>
        </el-collapse-item>
        <el-collapse-item title="输入">
          <el-row :gutter="20">
            <el-col :span="24">
              <el-form-item label="源">
                <el-radio-group v-model="input.type">
                  <el-radio-button label="url">链接</el-radio-button>
                  <el-radio-button label="file">本地文件</el-radio-button>
                </el-radio-group>
              </el-form-item>
            </el-col>
            <el-col :span="24" v-if="input.type === 'url'">
              <el-form-item label="URL">
                <el-input v-model="input.url"></el-input>
              </el-form-item>
            </el-col>
            <el-col :span="24" v-if="input.type === 'file'">
              <el-form-item label="输入文件">
                <el-input v-model="input.file" placeholder="请选择文件" @click="handleFileChange"></el-input>
              </el-form-item>
            </el-col>
          </el-row>
        </el-collapse-item>
      </el-collapse> <el-button type="primary" @click="handleSubmit">生成命令</el-button>
    </el-form>
  </div>
</template>

<script>
import { open } from '@tauri-apps/api/dialog';

export default {
  data() {
    return {
      activeNames: ['video', 'audio', 'input'],
      videoFormatOptions: [
        { label: 'AVI', value: 'avi' },
        { label: 'MP4', value: 'mp4' },
        { label: 'MOV', value: 'mov' }
      ],
      videoCodecOptions: [
        { label: 'H.264', value: 'h264' },
        { label: 'H.265', value: 'h265' },
        { label: 'MPEG-4', value: 'mpeg4' }
      ],
      audioFormatOptions: [
        { label: 'AAC', value: 'aac' },
        { label: 'MP3', value: 'mp3' },
        { label: 'WAV', value: 'wav' }
      ],
      audioSamplerateOptions: [
        { label: '44100 Hz', value: '44100' },
        { label: '48000 Hz', value: '48000' },
        { label: '96000 Hz', value: '96000' }
      ],
      video: {
        format: 'mp4',
        codec: 'h264',
        framerate: 25
      },
      audio: {
        format: 'aac',
        samplerate: '44100'
      },
      input: {
        type: 'url',
        url: '',
        file: null
      }
    };
  },
  methods: {
    handleSubmit() {
      let command = 'ffmpeg ';
      // Video options
      if (this.video.format) {
        command += `-f ${this.video.format} `;
      }
      if (this.video.codec) {
        command += `-c:v ${this.video.codec} `;
      }
      if (this.video.framerate) {
        command += `-r ${this.video.framerate} `;
      }

      // Audio options
      if (this.audio.format) {
        command += `-f ${this.audio.format} `;
      }
      if (this.audio.samplerate) {
        command += `-ar ${this.audio.samplerate} `;
      }

      // Input options
      if (this.input.type === 'url' && this.input.url) {
        command += `-i ${this.input.url} `;
      } else if (this.input.type === 'file' && this.input.file) {
        command += `-i ${this.input.file.path} `;
      }

      console.log(command);
    },
    handleFileChange() {
      open({
        multiple: true,
        filters: [{
          name: 'Video',
          extensions: ['mp4', 'mov']
        }]
      }).then((result) => {
        // console.log(result);
        if (result && result.length > 0) {
          this.input.file = result[0]
          // console.log(`result[0]:${result[0]},this.input.file:${this.input.file}`)
        }
      })
    }
  }
};
</script>