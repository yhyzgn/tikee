import { DeleteOutlined, PlusOutlined, ReloadOutlined } from '@ant-design/icons';
import { Button, Card, Form, Input, Modal, Select, Space, Table, Tag, Typography, message } from 'antd';
import { useEffect, useState } from 'react';

import { createCalendar, deleteCalendar, listAppScopes, listCalendars, listNamespaces, type AppScopeSummary, type CalendarSummary, type NamespaceSummary } from '../api/client';

interface CalendarFormValues {
  namespace: string;
  app: string;
  name: string;
  timezone: string;
  excludedDates?: string[];
  holidays?: string[];
  maintenanceWindowsText?: string;
  freezeWindowsText?: string;
}

function parseWindows(text?: string) {
  return String(text ?? '').split('\n').map((line) => line.trim()).filter(Boolean).map((line) => {
    const [start, end] = line.split(',').map((item) => item.trim());
    if (!start || !end) throw new Error(`窗口格式应为 start,end：${line}`);
    return { start, end };
  });
}

export function CalendarsPage() {
  const [items, setItems] = useState<CalendarSummary[]>([]);
  const [namespaces, setNamespaces] = useState<NamespaceSummary[]>([]);
  const [apps, setApps] = useState<AppScopeSummary[]>([]);
  const [loading, setLoading] = useState(false);
  const [open, setOpen] = useState(false);
  const [form] = Form.useForm<CalendarFormValues>();

  const reload = async () => {
    setLoading(true);
    try {
      const [calendars, namespaceItems, appItems] = await Promise.all([listCalendars(), listNamespaces(), listAppScopes()]);
      setItems(calendars);
      setNamespaces(namespaceItems);
      setApps(appItems);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { void reload(); }, []);

  const handleSubmit = async () => {
    const values = await form.validateFields();
    await createCalendar({
      namespace: values.namespace,
      app: values.app,
      name: values.name,
      timezone: values.timezone || 'UTC',
      excludedDates: values.excludedDates ?? [],
      holidays: values.holidays ?? [],
      maintenanceWindows: parseWindows(values.maintenanceWindowsText),
      freezeWindows: parseWindows(values.freezeWindowsText),
    });
    setOpen(false);
    form.resetFields();
    message.success('Calendar 已保存');
    await reload();
  };

  const handleDelete = async (id: string) => {
    await deleteCalendar(id);
    message.success('Calendar 已删除');
    await reload();
  };

  return (
    <Space direction="vertical" size={20} style={{ width: '100%' }}>
      <div>
        <Typography.Title level={2}>调度日历</Typography.Title>
        <Typography.Text type="secondary">集中维护 namespace/app 作用域下的节假日、维护窗口和冻结窗口；任务可通过 Calendar 引用绑定。</Typography.Text>
      </div>
      <Card extra={<Space><Button icon={<ReloadOutlined />} onClick={() => void reload()}>刷新</Button><Button type="primary" icon={<PlusOutlined />} onClick={() => { form.setFieldsValue({ namespace: 'default', app: 'default', timezone: 'Asia/Shanghai' }); setOpen(true); }}>新建 Calendar</Button></Space>}>
        <Table<CalendarSummary>
          rowKey="id"
          loading={loading}
          dataSource={items}
          columns={[
            { title: '名称', dataIndex: 'name' },
            { title: '范围', render: (_, item) => `${item.namespace}/${item.app}` },
            { title: '时区', dataIndex: 'timezone' },
            { title: '排除日期', render: (_, item) => <Space wrap>{[...item.excludedDates, ...item.holidays].map((date) => <Tag key={date}>{date}</Tag>)}</Space> },
            { title: '维护/冻结窗口', render: (_, item) => `${item.maintenanceWindows.length}/${item.freezeWindows.length}` },
            { title: '操作', width: 120, render: (_, item) => <Button danger size="small" icon={<DeleteOutlined />} onClick={() => void handleDelete(item.id)}>删除</Button> },
          ]}
        />
      </Card>
      <Modal title="新建/更新 Calendar" open={open} width={760} onOk={() => void handleSubmit()} onCancel={() => setOpen(false)} okText="保存">
        <Form form={form} layout="vertical">
          <Form.Item name="namespace" label="Namespace" rules={[{ required: true }]}><Select showSearch options={namespaces.map((item) => ({ value: item.name, label: item.name }))} /></Form.Item>
          <Form.Item name="app" label="App" rules={[{ required: true }]}><Select showSearch options={apps.map((item) => ({ value: item.name, label: `${item.namespace}/${item.name}` }))} /></Form.Item>
          <Form.Item name="name" label="名称" rules={[{ required: true }]}><Input placeholder="cn-maintenance" /></Form.Item>
          <Form.Item name="timezone" label="时区"><Input placeholder="Asia/Shanghai" /></Form.Item>
          <Form.Item name="excludedDates" label="排除日期"><Select mode="tags" placeholder="YYYY-MM-DD" /></Form.Item>
          <Form.Item name="holidays" label="节假日"><Select mode="tags" placeholder="YYYY-MM-DD" /></Form.Item>
          <Form.Item name="maintenanceWindowsText" label="维护窗口" extra="每行一个窗口：start,end；时间必须为 RFC3339。"><Input.TextArea rows={3} placeholder="2026-05-29T01:00:00Z,2026-05-29T02:00:00Z" /></Form.Item>
          <Form.Item name="freezeWindowsText" label="冻结窗口" extra="每行一个窗口：start,end；时间必须为 RFC3339。"><Input.TextArea rows={3} /></Form.Item>
        </Form>
      </Modal>
    </Space>
  );
}
