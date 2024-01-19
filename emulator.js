import { get_emulator_size } from "./constants.js";

export default class Emulator {
  get program_counter() {
    return this.exports.get_program_counter(this.emulator_ptr)
  }
  get vram() {
    let vram_ptr = this.exports.get_vram_ptr(this.emulator_ptr);
    let vram_arr = new Uint8Array(this._memory.buffer, vram_ptr, this.display_width * this.display_height);
    return Array.from(vram_arr)
  }
  get memory() {
    let memory_ptr = this.exports.get_memory_ptr(this.emulator_ptr);
    let memory_arr = new Uint8Array(this._memory.buffer, memory_ptr, this.memory_size);
    return Array.from(memory_arr)
  }
  constructor(instance) {
    this.exports = instance.exports
    this._memory = new DataView(instance.exports.memory.buffer)

    // get constants
    this.display_height = this.exports.get_display_height()
    this.display_width = this.exports.get_display_width()
    this.num_registers = this.exports.get_num_registers()
    this.memory_size = this.exports.get_memory_size()
    this.font_begin_addr = this.exports.get_font_begin_addr()
    this.program_begin_addr = this.exports.get_program_begin_addr()
    this.max_stack_size = this.exports.get_max_stack_size()
    this.num_keys = this.exports.get_num_keys()
    this.random_multipler = this.exports.get_random_multiplier()
    this.random_module = this.exports.get_random_module()
    this.random_increment = this.exports.get_random_increment()
    this.emulator_size = this.exports.get_emulator_size()

    this.emulator_ptr = this.alloc(this.emulator_size)
    this.exports.new(this.emulator_ptr);
  }
  should_beep() {
    return exports.should_beep(this.emulator_ptr);
  }
  alloc(len) {
    return this.exports.alloc(len)
  }
  copy_data_to_wasm(_data) {
    const data = new Uint8Array(_data);
    const len = data.byteLength;
    const ptr = this.alloc(len);
    const copy_arr = new Uint8Array(this._memory.buffer, ptr, len);
    copy_arr.set(data)
    return [ptr, len]
  }
  load_program(_rom) {
    const rom = new Uint8Array(_rom)
    const [rom_ptr, rom_len] = this.copy_data_to_wasm(rom)
    const result_ptr = this.alloc(8)

    let bu = new Uint8Array(this._memory.buffer, result_ptr)
    console.log(bu)
    this.exports.load_program(result_ptr, this.emulator_ptr, rom_ptr, rom_len);
    console.log(bu)

    const result_value = this._memory.getUint32(result_ptr);
    const success_flag = this._memory.getUint32(result_ptr + 4);
    return [result_value, success_flag]
  }
  get_opcode() {
    const result_ptr = this.alloc(8)

    let bu = new Uint8Array(this._memory.buffer, result_ptr)
    console.log(bu)

    this.exports.get_opcode(result_ptr, this.emulator_ptr);

    console.log(bu)
    const result_value = this._memory.getUint32(result_ptr);
    const success_flag = this._memory.getUint32(result_ptr + 4);
    return [result_value, success_flag]
  }
  tick(_keypad) {
    const keypad = new Uint8Array(_keypad);
    const [keypad_ptr, keypad_len] = this.copy_data_to_wasm(keypad)
    const result_ptr = this.alloc(2)
    this.exports.tick(result_ptr, this.emulator_ptr, keypad_ptr, keypad_len);
    const result_value = this._memory.getUint32(result_ptr);
    const success_flag = this._memory.getUint32(result_ptr + 4);
    return [result_value, success_flag]
  }
};
