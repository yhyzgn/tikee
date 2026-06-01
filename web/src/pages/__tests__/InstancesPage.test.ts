import { describe, expect, test } from 'bun:test';
import { readFileSync } from 'node:fs';

const source = readFileSync(new URL('../InstancesPage.tsx', import.meta.url), 'utf8');

describe('instance log drawer executor visibility', () => {
  test('shows executor details for single instances and broadcast child attempts separately', () => {
    expect(source).toContain("selectedInstance?.executionMode === 'single' ? '执行器' : '广播子执行'");
    expect(source).toContain("selectedInstance?.executionMode === 'single' ? [{");
    expect(source).toContain('workerId: selectedInstance.workerId ?? selectedInstance.latestLog?.workerId');
    expect(source).toContain('status: selectedInstance.status');
    expect(source).toContain('updatedAt: selectedInstance.updatedAt');
    expect(source).toContain('dataSource={selectedInstance?.executionMode === \'single\' ?');
    expect(source).toContain("'暂无执行器信息' : '暂无广播子执行'");
  });

  test('loads attempts and logs together and keeps worker/status columns visible', () => {
    expect(source).toContain('listInstanceAttempts(instance.id)');
    expect(source).toContain('listInstanceLogs(instance.id)');
    expect(source).toContain("{ title: 'Worker', dataIndex: 'workerId'");
    expect(source).toContain("{ title: 'Status', dataIndex: 'status'");
    expect(source).toContain("{ title: 'Updated At', dataIndex: 'updatedAt'");
    expect(source).toContain("{ title: 'Worker', dataIndex: 'workerId', ellipsis: true, width: 120 }");
    expect(source).toContain('查看日志');
  });
});
