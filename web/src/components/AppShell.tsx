import { DashboardOutlined, DeploymentUnitOutlined, LogoutOutlined, UnorderedListOutlined } from '@ant-design/icons';
import { Button, Layout, Menu, Space, Typography, theme } from 'antd';
import type { ReactNode } from 'react';

const { Header, Sider, Content } = Layout;

export interface AppShellProps {
  children: ReactNode;
  activeKey: string;
  username: string;
  onNavigate: (key: string) => void;
  onLogout: () => void;
}

export function AppShell({ children, activeKey, username, onNavigate, onLogout }: AppShellProps) {
  const { token } = theme.useToken();

  return (
    <Layout className="app-shell">
      <Sider breakpoint="lg" collapsedWidth="0" className="app-shell__sider">
        <div className="app-shell__brand">scheduler</div>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={[activeKey]}
          onClick={(event) => onNavigate(event.key)}
          items={[
            { key: 'dashboard', icon: <DashboardOutlined />, label: 'Dashboard' },
            { key: 'jobs', icon: <UnorderedListOutlined />, label: 'Jobs' },
            { key: 'instances', icon: <DeploymentUnitOutlined />, label: 'Instances' },
          ]}
        />
      </Sider>
      <Layout>
        <Header className="app-shell__header">
          <Typography.Title level={4} style={{ margin: 0, color: token.colorTextHeading }}>
            分布式任务调度平台
          </Typography.Title>
          <Space className="app-shell__user">
            <Typography.Text type="secondary">{username}</Typography.Text>
            <Button icon={<LogoutOutlined />} onClick={onLogout}>
              退出
            </Button>
          </Space>
        </Header>
        <Content className="app-shell__content">{children}</Content>
      </Layout>
    </Layout>
  );
}
