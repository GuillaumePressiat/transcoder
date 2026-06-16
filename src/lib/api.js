import { invoke } from '@tauri-apps/api/core';

/**
 * @typedef {'list' | 'sql' | 'pipe'} OutputType
 * @typedef {'single' | 'double' | 'none'} Quote
 * @typedef {'comma_space' | 'space' | 'comma'} Separator
 * @typedef {{ output: string, count: number }} TranscodeResult
 */

/**
 * @param {string} input
 * @param {OutputType} output_type
 * @param {Quote} quote
 * @param {Separator} separator
 * @returns {Promise<TranscodeResult>}
 */
export function transcodeLine(input, outputType, quote, separator) {
  return invoke('transcode_line', { input, outputType, quote, separator });
}

/**
 * @param {string} input
 * @param {OutputType} output_type
 * @param {Quote} quote
 * @param {Separator} separator
 * @returns {Promise<TranscodeResult>}
 */
export function transcodeColumn(input, outputType, quote, separator) {
  return invoke('transcode_column', { input, outputType, quote, separator });
}

/**
 * @param {string} input
 * @param {'column_to_line' | 'line_to_column'} direction
 * @param {Separator} separator
 * @returns {Promise<TranscodeResult>}
 */
export function convertFormat(input, direction, separator) {
  return invoke('convert_format', { input, direction, separator });
}

/**
 * @param {string} text
 * @returns {Promise<void>}
 */
export function copyToClipboard(text) {
  return invoke('copy_to_clipboard', { text });
}
