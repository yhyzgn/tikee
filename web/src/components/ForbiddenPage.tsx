import { Button, Result } from 'antd';
import { useNavigate } from 'react-router-dom';

export function ForbiddenPage() {
  const navigate = useNavigate();
  return (
    <Result
      status="403"
      title="403"
      subTitle="当前账号没有访问该功能的权限"
      extra={<Button type="primary" onClick={() => navigate('/dashboard', { replace: true })}>返回总览</Button>}
    />
  );
}
