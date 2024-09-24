import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { RobotJs } from '../index';

describe('RobotJs', () => {
  let robot: RobotJs;

  beforeEach(() => {
    robot = new RobotJs();
  });

  afterEach(() => {
    // Clean up if necessary
  });

  it('should create a RobotJs instance', () => {
    expect(robot).toBeInstanceOf(RobotJs);
  });

  it('should get mouse position', () => {
    const pos = robot.getMousePos();
    expect(pos).toHaveProperty('x');
    expect(pos).toHaveProperty('y');
    expect(typeof pos.x).toBe('number');
    expect(typeof pos.y).toBe('number');
  });

  it('should move mouse', () => {
    const result = robot.moveMouse(100, 100);
    expect(result).toBe(1);

    const newPos = robot.getMousePos();
    expect(newPos.x).toBe(100);
    expect(newPos.y).toBe(100);
  });

  it('should get screen size', () => {
    const size = robot.getScreenSize();
    expect(size).toHaveProperty('width');
    expect(size).toHaveProperty('height');
    expect(typeof size.width).toBe('number');
    expect(typeof size.height).toBe('number');
    expect(size.width).toBeGreaterThan(0);
    expect(size.height).toBeGreaterThan(0);
  });

  it('should get pixel color', () => {
    const color = robot.getPixelColor(0, 0);
    expect(color).toMatch(/^#[0-9A-Fa-f]{6}$/);
  });

  it('should set mouse delay', () => {
    const result = robot.setMouseDelay(20);
    expect(result).toBe(1);
  });

  it('should set keyboard delay', () => {
    const result = robot.setKeyboardDelay(20);
    expect(result).toBe(1);
  });

  it('should type string', () => {
    const result = robot.typeString('Hello, RobotJS!');
    expect(result).toBe(1);
  });

  it('should capture screen', () => {
    const screenshot = robot.captureScreen();
    expect(screenshot).toHaveProperty('width');
    expect(screenshot).toHaveProperty('height');
    expect(screenshot).toHaveProperty('image');
    expect(screenshot.image instanceof Uint8Array).toBe(true);
  });

  it('should get color from captured image', () => {
    const screenshot = robot.captureScreen();
    const color = robot.getColor(screenshot, 0, 0);
    expect(color).toMatch(/^#[0-9A-Fa-f]{6}$/);
  });

  // Note: The following tests might interfere with user's mouse/keyboard
  // Consider running these tests in a controlled environment

  it('should perform mouse click', () => {
    const result = robot.mouseClick();
    expect(result).toBe(1);
  });

  it('should perform key tap', () => {
    const result = robot.keyTap('a');
    expect(result).toBe(1);
  });
});