import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { DashboardLoading } from '../components/dashboard/DashboardLoading';
import { useSubscribe } from '../webVault/useSubscribe';
import { useWebVault } from '../webVault/useWebVault';

export const LoginPage: React.FC<{}> = () => {
  const webVault = useWebVault();
  const navigate = useNavigate();
  const oauth2Status = useSubscribe(
    (v, cb) => v.oauth2StatusSubscribe(cb),
    (v) => v.oauth2StatusData,
    []
  );

  useEffect(() => {
    if (oauth2Status.type === 'Loaded') {
      navigate('/', {
        replace: true,
      });
    } else {
      document.location.href = webVault.oauth2StartFlow();
    }
  }, [webVault, oauth2Status, navigate]);

  return <DashboardLoading />;
};
